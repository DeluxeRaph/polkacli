use polkacli::run_command;
use polkacli::Commands;

#[tokio::test]
async fn test_balance() {
    let command = Commands::Balance { 
        address: Some("14xmwinmCEz6oRrFdczHKqHgWNMiCysE2KrA4jXXAAM1Eogk".to_string()) 
    };

    let result = run_command(command).await;

    assert!(result.is_ok(), "Balance Command failed")
}
