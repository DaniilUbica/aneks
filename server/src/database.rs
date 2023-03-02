pub mod database {
    const TABLE_NAME: &str = "users";

    fn connect_to_db() -> sqlite::Connection {
        let connection = sqlite::open("DB_USERS.db").expect("Can't open database");
        connection
    }

    pub fn create_table(table_name: &str, columns: Vec<&str>) {
        
        let column_names: String = columns.into_iter().collect(); // name TEXT, age INT, sirname TEXT

        let query = format!("CREATE TABLE IF NOT EXISTS {table_name}({column_names})");

        let connection = connect_to_db();
        connection.execute(query).expect("Error in executing the query!");
    }

    pub fn add_record(columns: Vec<&str>) {
        let email = columns[0];

        let query = format!("INSERT INTO {TABLE_NAME} VALUES ('{email}')");

        let connection = connect_to_db();
        connection.execute(query).expect("Error in executing the query!");
    }

    pub fn get_record(values: &Vec<&str>) -> Vec<String> {
        let email = values[0];
        let mut answer: Vec<String> = Vec::new();

        let query = format!("SELECT DISTINCT * FROM {TABLE_NAME} WHERE email = '{email}'");

        let connection = connect_to_db();

        connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                let s = format!("{}", value.unwrap());
                answer.push(s);
            }
            true
        })
        .unwrap();

        answer
    }
}