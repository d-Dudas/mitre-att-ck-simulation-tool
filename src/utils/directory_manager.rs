pub struct DirectoryManager
{
    directory_path: String,
}

impl DirectoryManager
{
    pub fn new(directory_path: &String) -> Self
    {
        DirectoryManager {
            directory_path: directory_path.clone(),
        }
    }

    pub fn directory_exists(&self) -> bool
    {
        std::path::Path::new(&self.directory_path).exists()
    }

    pub fn create_directory(&self) -> std::io::Result<()>
    {
        std::fs::create_dir_all(&self.directory_path)
    }

    pub fn get_global_path(&self) -> String
    {
        let current_dir = std::env::current_dir().unwrap();
        let global_path = current_dir.join(&self.directory_path);

        global_path.to_str().unwrap().to_string()
    }
}
