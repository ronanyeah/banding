use anchor_client::solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_id: Pubkey = "5WM4KkVsMm6TGnJTyLrSikxT8rszdhRGi9AGgAJZWvqR".parse()?;
    let client = anchor_client::Client::new("devnet".parse()?, std::rc::Rc::new(Keypair::new()));
    let program = client.program(program_id);
    let (data, _) = Pubkey::find_program_address(&[b"state"], &program_id);

    let res = program.account::<banding::State>(data)?;

    println!("{:#?}", res);

    Ok(())
}
