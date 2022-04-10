
pub mod user_inner{
    use std::cell::RefCell;
    use std::rc::Rc;

    use yew::use_state;

    pub type User = Rc<UserInner>;

    #[derive(Debug, PartialEq)]
    pub struct UserInner {
        pub username: RefCell<String>,
    }

    pub fn get_user_state() -> yew::UseStateHandle<Rc<UserInner>> {
        use_state(|| {
            Rc::new(UserInner {
                username: RefCell::new("initial".into()),
            })
        })
    }
}
