
#include <sys/ptrace.h>
#include <sys/syscall.h>
#include <errno.h>
#include <stdint.h>
#include <sys/wait.h>
#include <stdio.h> // remove this when you make the production version
#include <inttypes.h>
#include <stdbool.h>
//STRUCTS
struct FromAddress{
	uint8_t error_code;
	uint8_t data;
	uint32_t data32;
	uint64_t data64;

};
//END OF STRUCTS







//checks if the process exists
int check_process_exists(int pid) {
    if (waitpid(pid, NULL, WNOHANG) == 0) {
        return 1;
    }
    return 0;
}


//converts the 64 bit value to the array of 8 bytes
void convert_64bit_to_byteslittle(uint64_t value, uint8_t *bytes) {
    for (int i = 0; i < 8; i++) {
        bytes[i] = (value >> (8 * i)) & 0xFF;
    }
}

//combines the array of 4 words to a 64 bit value, its little endain
uint64_t combine_array_to_64bitlittle(uint16_t data_array[3]) {
	
	bool can_null=false;
	bool null_rest = false; 

	for(int buffer=0;buffer!=3;buffer++) {
	
		if(null_rest){
			
			data_array[buffer] = 0;


		}

		else{





			if (data_array[buffer] != 0) {
			
				can_null = true;

			}

			if(can_null && data_array[buffer] == 0) {
			
				null_rest = true;


			}

		}

	}

    uint64_t combined_Value = ((uint64_t)data_array[3] << 48) |
                           ((uint64_t)data_array[2]<< 32) |
                           ((uint64_t)data_array[1] << 16) |
                           (uint64_t)data_array[0];

	return combined_Value;


}

//converts a 32 bit value to 4 bytes, is little endain
void convert_32bit_to_byteslittle(uint32_t data,uint8_t *data_array) {


	data_array[0] = data & 0xFF;

    data_array[1] = (data >> 8) & 0xFF;

    data_array[2] = (data >> 16) & 0xFF;

    data_array[3] = (data >> 24) & 0xFF;

}



//combines the array of 4 bytes to a 32 bit value, is little endain
uint32_t combine_array_to_32bitlittle(uint8_t *data_array) {

	
	return (data_array[3] << 24) | (data_array[2] << 16) | data_array[1] | data_array[0];



}


/*
attaches to the process, the process is pointed to by the first parameter which should be the id of the process.
returns 1 if it failed to attach 2 if it failed because of invalid perms or 3 it failed because no such process exists, returns 1 if it successfully attached.
*/
 //for the record!!!!!!!!!!!! when the process isnt paused you cant read from it
char attach_process(int pid){

	

	//ptrace(PTRACE_SEIZE,pid,0,0);
	ptrace(PTRACE_ATTACH,pid,0,0);
	//waitpid(pid);
	//printf("attached\n");
	//ptrace(PTRACE_CONT,pid,0,0);
	//ptrace(PTRACE_INTERRUPT,pid,0,0);

	//attach_error();
	

	switch(errno){

		case EPERM:
			return 2;

		case ESRCH:
			return 3;

		default:
			return 0;
	}
	
	return 1;
	

}

//forces the process to continue if it has frozen for any reason 
void force_continue(int pid) {
	
	ptrace(PTRACE_CONT,pid,0,0);

	return;

}

/*
outputs the ptrace attach log into stdout, may remove this in later versions
*/
void attach_error(){

	perror("attach");
}


/*
detaches a process, the process should go into the pid parameter.
*/
char dettach_process(int pid){

	ptrace(PTRACE_DETACH,pid,0,0);

	return 0;

}


/*
outputs the ptrace attach log into stdout, may remove this in later versions
*/
void dettach_error(){

	perror("dettach");
}
/* 
reads an address and returns the byte from the address, returns error_code 1 in the struct if the address doesnt exist/wasnt allocated
*/
struct FromAddress read_address8(int pid,int* address){
	struct FromAddress from_address;
	
	//ptrace(PTRACE_ATTACH,pid,0,0);

	from_address.data=ptrace( PTRACE_PEEKDATA, pid, address, 0 );

	
	switch(errno){
		case EFAULT:
			from_address.error_code=1;
			break;

		case EIO:
			from_address.error_code=1;
			break;

		default:
			from_address.error_code=0;
			break;		
	}

	return from_address;
	
}
/* 
reads an address and returns the long from the address, returns error_code 1 in the struct if the address doesnt exist/wasnt allocated. is little endain
*/
struct FromAddress read_address32little(int pid,int* address){

	struct FromAddress from_address={0};
	
	/*
	uint8_t data_array[3]={0};

	data_array[0] = 1;

	for(int buffer=0;buffer<=3;buffer++){
		
		data_array[buffer]=ptrace(PTRACE_PEEKDATA,pid,address+buffer,0);
		
		//printf("%02hhx || %d \n ~~ ad 0x%" PRIx64 "\n",data_array[buffer],data_array[buffer],address);

		switch(errno){
			case EFAULT:
				// from_address.error_code=1;
				return from_address;
				break;

			case EIO:
			    // from_address.error_code=1;
				return from_address;
				break;

			default:
				// from_address.error_code=0;
				break;
		}

		switch(data_array[buffer]) {

		
			case 0:
				
				from_address.data32 = combine_array_to_32bitlittle(data_array);

				return from_address;

				break;

			defualt:

				break;


		}

		
	}


	*/
	
	//from_address.data32=combine_array_to_32bitlittle(data_array);

	from_address.data32 = ptrace(PTRACE_PEEKDATA,pid,address,0);

	//from_address.data32=(data_array[0] << 24) | (data_array[1] << 16) | (data_array[2] << 8) | data_array[3];
	
	//printf("got %d",from_address.data32);

	return from_address;
}
/* 
reads an address and returns the long from the address, returns error_code 1 in the struct if the address doesnt exist/wasnt allocated. is little endain
*/
struct FromAddress read_address64little(int pid,int* address){

	//printf("reading on %llu",address);
	
	struct FromAddress from_address={0};
	
	/*

	uint16_t data_array[3]={0};


	

	for(int buffer=0;buffer<=3;buffer++){
		
		data_array[buffer]=ptrace(PTRACE_PEEKDATA,pid,address+buffer,0);
			
		//read_error();
		
		printf(">> %x\n",data_array[buffer]);
		
		

		switch(errno){
			case EFAULT:
				from_address.error_code=1;
				return from_address;
				break;

			case EIO:
				from_address.error_code=1;
				return from_address;
				break;

			default:
				from_address.error_code=0;
				break;
		}


		//printf("@@K! %hhu %u\n",data_array[buffer],buffer);
		/*
		switch(data_array[buffer]) {

		
			case 0:
				
				from_address.data64 = combine_array_to_64bitlittle(data_array);

				return from_address;

				break;

			defualt:

				break;


		}
		
		//printf("@@K %hhu %u\n",read_address8(pid,address+buffer).data,buffer);

		
		
	} */

	//from_address.data64 = combine_array_to_64bitlittle(data_array);
	

	
	from_address.data64 = ptrace(PTRACE_PEEKDATA,pid,address,0);

	switch(errno){
			case EFAULT:
				from_address.error_code=1;
				return from_address;
				break;

			case EIO:
				from_address.error_code=1;
				return from_address;
				break;

			default:
				from_address.error_code=0;
				break;
	}


	//from_address.data32=(data_array[0] << 24) | (data_array[1] << 16) | (data_array[2] << 8) | data_array[3];
	
	//printf("got %d",from_address.data32);

	return from_address;
}
/*
outputs the logs from read_from_address into stdout, may be removed in later versions
*/
void read_error(){

	perror("peekdata");

}

/*
Writes the data in the second parameter to the address contained in the third parameter
*/
uint8_t write_address8(int pid,int* address,uint8_t data){

	ptrace(PTRACE_POKEDATA,pid,address,data);

	switch(errno){
		case EFAULT:
			return 1;

		case EIO:
			return 1;

		default:
			return 0;

	}

	return 1;


}

/*
Writes the data in the second parameter to the address contained in the third parameter
*/
uint8_t write_address32little(int pid,int* address,uint32_t data){
	uint8_t data_array[3];
	
	/*
	convert_32bit_to_byteslittle(data,data_array);

	for(int buffer=0;buffer<3;buffer++){
		ptrace(PTRACE_POKEDATA,pid,address+buffer,data_array[buffer]);

		switch(errno){

			case EFAULT:
				return 1;
				break;

			case EIO:
				return 1;
				break;

			default:
				break;		
		}
	}
	*/
	
	ptrace(PTRACE_POKEDATA,pid,address,data);

	switch(errno){

			case EFAULT:
				return 1;
				break;

			case EIO:
				return 1;
				break;

			default:
				break;		
		}
	

	return 0;
}

/*
Writes the data in the second parameter to the address contained in the third parameter
*/
uint8_t write_address64little(int pid,int* address,uint64_t data){
	uint8_t data_array[7];
	
	/*
	convert_64bit_to_byteslittle(data,data_array);
	
	for(int buffer=0;buffer<7;buffer++){
		ptrace(PTRACE_POKEDATA,pid,address+buffer,data_array[buffer]);

		switch(errno){

			case EFAULT:
				return 1;
				break;

			case EIO:
				return 1;
				break;

			default:
				break;		
		}
	}
	return 0;
	*/

	ptrace(PTRACE_POKEDATA,pid,address,data);

	switch(errno){

			case EFAULT:
				return 1;
				break;

			case EIO:
				return 1;
				break;

			default:
				break;		
		}
	

	return 0;

}



