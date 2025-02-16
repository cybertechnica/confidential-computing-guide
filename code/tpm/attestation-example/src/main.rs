
use tss_esapi::{
    Context,
    TctiNameConf,
};
pub mod attestation;
use crate::attestation::run_attestation;


fn main() {
    println!("Remote Attestation example : ");

    let mut context = Context::new(
        TctiNameConf::from_environment_variable()
            .expect("Failed to get TCTI / TPM2TOOLS_TCTI from environment. Try `export TCTI=device:/dev/tpmrm0`"),
    )
    .expect("Failed to create Context");

    run_attestation(&mut context).unwrap();

}
