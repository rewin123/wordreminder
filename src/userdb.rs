use crate::user::User;

#[derive(Clone)]
pub struct UserDB
{
    dir_path : String
}

impl Default for UserDB {
    fn default() -> Self {
       let res = UserDB { dir_path: "users".to_string() };
       res.check_folder();
       res
    }
}

impl UserDB {

    pub fn check_folder(&self) {
        std::fs::create_dir(self.dir_path.as_str());
    }

    pub fn get_user_path(&self, id : i32) -> String {
        format!("{}/{}.ron", self.dir_path, id)
    }

    pub fn get_user(&self, id : i32) -> User {

        let read_path = self.get_user_path(id);
        match std::fs::read_to_string(read_path) {
            Ok(content) => {
                match ron::from_str(content.as_str()) {
                    Ok(user) => user,
                    Err(_) => {

                    },
                }
            }
            Err(_) => {

            }
        }

        User { id: id, words: vec![] }
    }

    pub fn set_user(&self, user : &User) {
        // let content = serde_json::to_string_pretty(user).unwrap();
        
        let content = ron::ser::to_string_pretty(user, ron::ser::PrettyConfig::default()).unwrap();
        let path = self.get_user_path(user.id);
        std::fs::write(path, content).unwrap();
    }
}