#[test]
fn test_draw_welcome_page() {
    let mut terminal = lazyarchbuild::setup_crossterm_terminal().unwrap();

    lazyarchbuild::canvas::draw_welcome_page(&mut terminal).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    lazyarchbuild::clean_up_terminal(&mut terminal).unwrap();
}
