mod users {
    
    pub struct User{
        pub name: String
    }

}
mod Admins;

use crate::Admins::AddAdmin;
use users::normalUsers;

fn dosomething(){
    let usrs = users::User{name:String::from("Ansh")};
    AddAdmin();
}