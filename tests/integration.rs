use solana_program_test::{processor, tokio, ProgramTest};

fn program_test() -> ProgramTest {
    ProgramTest::new(
        "gateway",
        gateway::id(),
        processor!(gateway::processor::process_instruction),
    )
}

#[tokio::test]
async fn test_queue_message() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
}
