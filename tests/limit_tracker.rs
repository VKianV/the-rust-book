use the_book::{LimitTracker, Messenger}; // <-- your library

struct MockMessenger {
    sent_messages: std::cell::RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> Self {
        Self {
            sent_messages: std::cell::RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    assert_eq!(
        mock_messenger.sent_messages.borrow()[0],
        "Warning: You've used up over 75% of your quota!"
    );
}
