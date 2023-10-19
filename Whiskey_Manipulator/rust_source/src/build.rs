fn main(){
    println!("cargo:rerun-if-changed=build.rs"); // Re-run if the build script itself changes
    println!("cargo:rerun-if-changed=../c_source/main.c"); // Re-run if the main source file changes
    println!("cargo:rerun-if-env-changed=MY_ENV_VAR");
    cc::Build::new().file("/home/tony/project_folder/whiskey/Whiskey_Manipulator/c_source/main.c").compile("whiskc");

}
