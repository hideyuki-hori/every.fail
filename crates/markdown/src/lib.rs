#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Heading { level: u8, content: String },
}

pub fn parse(input: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    
    for line in input.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count() as u8;
            let content = trimmed.trim_start_matches('#').trim().to_string();
            nodes.push(Node::Heading { level, content });
        }
    }
    
    nodes
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_heading() {
       let input = "# Hello";
       let result = parse(input);
       assert_eq!(result, vec![Node::Heading { level: 1, content: "Hello".to_string() }]);

       let input = "## Hello";
       let result = parse(input);
       assert_eq!(result, vec![Node::Heading { level: 2, content: "Hello".to_string() }]);

       let input = "### Hello";
       let result = parse(input);
       assert_eq!(result, vec![Node::Heading { level: 3, content: "Hello".to_string() }]);
   }
}
