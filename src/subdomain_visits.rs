use std::collections::HashMap;

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut ans = vec![];
    if cpdomains.len() == 0 {
        return ans;
    }

    let mut visits: HashMap<String, usize> = HashMap::new();

    for pair in cpdomains.iter() {
        let record: Vec<&str> = pair.split_whitespace().collect();
        let freq: usize = record[0].parse().unwrap();

        let mut domain = record[1];
        while let Some(idx) = domain.find(".") {
            if !visits.contains_key(domain) {
                visits.insert(domain.to_string(), 0);
            }
            visits.insert(domain.to_string(), visits.get(domain).unwrap() + freq);
            domain = &domain[(idx + 1)..];
        }

        if !visits.contains_key(domain) {
            visits.insert(domain.to_string(), 0);
        }
        visits.insert(domain.to_string(), visits.get(domain).unwrap() + freq);
    }

    for (key, value) in &visits {
        ans.push(format!("{} {}", value, key));
    }
    return ans;
}

fn main() {
    assert_eq!(
        subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]),
        vec!["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
    )
}
