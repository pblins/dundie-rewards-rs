use std::collections::HashMap;

pub fn join_filters(dept: &Option<String>, email: &Option<String>) -> HashMap<String, String> {
    let mut query: HashMap<String, String> = HashMap::new();

    match dept {
        Some(dept_value) => query.insert("dept".to_string(), dept_value.to_string()),
        _ => None,
    };

    match email {
        Some(email_value) => query.insert("email".to_string(), email_value.to_string()),
        _ => None,
    };

    query
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::utils::db::join_filters;

    #[rstest]
    fn join_filters_only_email() {
        let dept = &None;
        let email = &Some("t@t.com".to_string());

        let query = join_filters(dept, email);

        assert_eq!(query.keys().count(), 1);
        assert_eq!(query.keys().last().unwrap(), "email");
        assert_eq!(query.values().count(), 1);
        assert_eq!(query.values().last().unwrap(), "t@t.com");
    }

    #[rstest]
    fn join_filters_only_dept() {
        let email = &None;
        let dept = &Some("Sales".to_string());

        let query = join_filters(dept, email);

        assert_eq!(query.keys().count(), 1);
        assert_eq!(query.keys().last().unwrap(), "dept");
        assert_eq!(query.values().count(), 1);
        assert_eq!(query.values().last().unwrap(), "Sales");
    }
}
