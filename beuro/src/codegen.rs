use crate::nodes::NodeExit;

pub struct CodeGenerator {
    root: NodeExit,
}

impl CodeGenerator {
    pub fn new(root: NodeExit) -> Self {
        Self { root }
    }

    pub fn generate_asm(self) -> String {
        let mut output: String = String::new();
        output.push_str("global _start\n_start:\n");

        output.push_str(&format!(
            "    mov rax, 60\n    mov rdi, {}\n    syscall\n",
            self.root.expr.int_literal.token_value.as_ref().unwrap()
        ));

        output
    }
}
