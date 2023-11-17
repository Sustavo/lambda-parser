use std::io;

#[derive(Debug, PartialEq, Clone)] 
enum Tipo {
    Nat,
    Bool,
    Seta(Box<Tipo>, Box<Tipo>),
    SemTipo,
}

#[derive(Debug, PartialEq, Clone)] 
enum Var {
    Var(String),
}

#[derive(Debug, PartialEq, Clone)] 
enum Termo {
    True,
    False,
    Numero(i32),
    Suc,
    Pred,
    EhZero,
    Var(Box<String>),
    If(Box<Termo>, Box<Termo>, Box<Termo>),
    Aplicacao(Box<Termo>, Box<Termo>),
    Lambda(Box<Var>, Box<Tipo>, Box<Termo>),
}

fn vector_tail<T: Clone>(list: Vec<T>) -> Vec<T> {
    let mut new_vector: Vec<T> = vec![];
    let indexes = list.len();
    for i in 1..indexes {
        new_vector.push(list[i].clone());
    }

    return new_vector;
}



fn type_variable<'a>(var: &'a str, lista: Vec<(&'a str, Tipo)>) -> Tipo {
    if lista.is_empty() {
        return Tipo::SemTipo;
    }

    for (variavel, tipo) in lista {
        if variavel == var {
            return tipo;
        }
    }

    return Tipo::SemTipo;
}


fn type_values(termo: Termo, lista: Vec<(&str, Tipo)>) -> Tipo {
    match termo {
        Termo::True => Tipo::Bool,
        Termo::False => Tipo::Bool,
        Termo::Numero(_) => Tipo::Nat,
        Termo::Suc => Tipo::Seta(Box::new(Tipo::Nat), Box::new(Tipo::Nat)),
        Termo::Pred => Tipo::Seta(Box::new(Tipo::Nat), Box::new(Tipo::Nat)),
        Termo::EhZero => Tipo::Seta(Box::new(Tipo::Nat), Box::new(Tipo::Bool)),
        Termo::Var(x) => type_variable(&x, lista),
        Termo::If(conditional, then_branch, else_branch) => {
            let type_cond = type_values(*conditional, lista.clone());
            let type_then = type_values(*then_branch, lista.clone());
            let type_else = type_values(*else_branch, lista.clone());

            if (type_cond == Tipo::Bool) && (type_then == type_else) {
                return type_then;
            }
            return Tipo::SemTipo;
        }
        Termo::Aplicacao(first_item, second_item) => {
            let primeiro_valor = type_values(*first_item, lista.clone());
            let segundo_valor = type_values(*second_item, lista.clone());
            match (primeiro_valor, segundo_valor) {
                (Tipo::Seta(u, b), c) => {
                    if *u == c {
                        return *b;
                    } else {
                        return Tipo::SemTipo;
                    }
                }
                _ => Tipo::SemTipo,
            }
        }
        Termo::Lambda(variavel, tipo, termo) => {
            let mut enviar = lista;
            let extract_string = match &*variavel {
                Var::Var(string) => string.clone(),
            };
            enviar.push((&*extract_string.as_str() , *tipo.clone()));
            return Tipo::Seta(tipo, Box::new(type_values(*termo, enviar)));
        }
    }
}

fn pass_to_type(tipo: Vec<&str>) -> (Tipo, Vec<&str>) {
    if tipo.is_empty() { panic!("!") };

    for variable in &tipo {
        let first_tail_vector = vector_tail(tipo.clone());
        match *variable {
            "(" => {
                let (type_one, first_vector_string) = pass_to_type(first_tail_vector);
                let (type_two, second_vector_string) = pass_to_type(first_vector_string);
                if second_vector_string.is_empty() { panic!("!") };
                
                for second_variable in &*second_vector_string {
                    let second_tail_vector = vector_tail(second_vector_string.clone());
                    match *second_variable {
                        ")" => {
                            return (Tipo::Seta(
                                Box::new(type_one.clone()),
                                Box::new(type_two.clone()),
                            ),  second_tail_vector)
                        }
                        _ => panic!("!"),
                    };
                }
            }
            "->"  => {
                let (result_type, remaining_vector) = pass_to_type(first_tail_vector);
                return (result_type, remaining_vector)
            }
            "Bool" => {
                return (Tipo::Bool, first_tail_vector);
            }
            "Nat" => {
                return (Tipo::Nat, first_tail_vector);
            }
            _ => panic!("!"),
        }
    };

    unreachable!();
}

fn invalidator(variable: &str) -> bool {
    match variable {
        "true" | "false" | "if" | "then" | "else" | "endif" | "suc" | "pred"
        | "ehzero" | "lambda" | "Nat" | "Bool" | "End" => true,
        _ => false,
    }
}

fn pass_to_term(symbols: Vec<&str>) -> (Termo, Vec<&str>) {
    if symbols.is_empty() { panic!("!") };

    for symbol in &symbols {
        let tail_vector_one = vector_tail(symbols.clone());
        match *symbol {
            "(" => {
                let (term_one, vector_string_one) = pass_to_term(tail_vector_one);
                let (term_two, vector_string_two) = pass_to_term(vector_string_one);
                if vector_string_two.is_empty() { panic!("!") };
                for variable in &*vector_string_two {
                    let tail_vector_two = vector_tail(vector_string_two.clone());
                    match *variable {
                        ")" => {
                            return (Termo::Aplicacao(
                                Box::new(term_one.clone()),
                                Box::new(term_two.clone()),
                            ),  tail_vector_two)
                        }
                        _ => panic!("!"),
                    };
                }
            }
            ")" => panic!("!"),
            "if" => {
                let (term_one, vector_string_one) = pass_to_term(tail_vector_one);
                if vector_string_one.is_empty() { panic!("!") };

                for variable_one in &*vector_string_one {
                    let tail_vector_two = vector_tail(vector_string_one.clone());
                    match *variable_one {
                        "then" => {
                            let (term_two, vector_string_two) = pass_to_term(tail_vector_two);
                            if vector_string_two.is_empty() { panic!("!") };

                            for variable_two in &*vector_string_two {
                                let tail_vector_three = vector_tail(vector_string_two.clone());
                                match *variable_two {
                                    "else" => {
                                        let (term_three, vector_string_three) = pass_to_term(tail_vector_three);
                                        if vector_string_three.is_empty() { panic!("!") };
                                        for variable_three in &*vector_string_three {
                                            let tail_vector_four = vector_tail(vector_string_three.clone());
                                            match *variable_three {
                                                "endif" => {
                                                    return (Termo::If(
                                                        Box::new(term_one.clone()),
                                                        Box::new(term_two.clone()),
                                                        Box::new(term_three.clone()),
                                                    ), tail_vector_four)
                                                }
                                                _ => panic!("!"),
                                            };
                                        }

                                    }
                                    _ => panic!("!"),
                                }
                            }


                        }
                        _ => panic!("!"),
                    }
                }

            }
            "true" => {
               return (Termo::True, tail_vector_one)
            },
            "false" => {
                return (Termo::False, tail_vector_one)
             },
             "suc" => {
                return (Termo::Suc, tail_vector_one)
             },
             "ehzero" => {
                return (Termo::EhZero, tail_vector_one)
             },
             "pred" => {
                return (Termo::Pred, tail_vector_one)
             },
             "lambda"=> {
                let var = tail_vector_one[0];
                let rest = vector_tail(vector_tail(tail_vector_one));

                if invalidator(var) {
                    panic!("!")
                } else { 
                    let (type_one, string_vector_one) = pass_to_type(rest);
                    for variable_one in &*string_vector_one {
                        let tail_vector_one = vector_tail(string_vector_one.clone());
                        match *variable_one {
                            "." => {
                                let (term_one, string_vector_two) = pass_to_term(tail_vector_one);
                                for variable_two in &*string_vector_two {
                                    let tail_vector_two = vector_tail(string_vector_two.clone());
                                    match *variable_two {
                                        "end" => {
                                            return (Termo::Lambda(
                                                Box::new(Var::Var(var.to_string())),
                                                Box::new(type_one.clone()),
                                                Box::new(term_one.clone()),
                                            ), tail_vector_two)
                                        }
                                        _ => panic!("!"),
                                    };
                                }
                            },
                            _ => panic!("!"),
                        }

                    }

                }
            },

            _ => {
                if let Ok(num) = symbol.parse::<i32>() {
                    return (Termo::Numero(num), tail_vector_one);
                } else {
                    if invalidator(symbol) {
                        panic!("!");
                    } else {
                        return (Termo::Var(Box::new(symbol.to_string())), tail_vector_one);
                    }
                }
            },
        }

    }
    
    unreachable!();
}

fn print_result(tipo: &Tipo) {
    fn string_of_tipo(tipo: &Tipo) -> String {
        match tipo {
            Tipo::Bool => "Bool".to_string(),
            Tipo::Nat => "Nat".to_string(),
            Tipo::Seta(a, b) => {
                let t1 = string_of_tipo(a);
                let t2 = string_of_tipo(b);
                format!("( {} -> {} )", t1, t2)
            }
            Tipo::SemTipo => panic!("-"),
        }
    }

    let texto = string_of_tipo(tipo);
    println!("{}", texto);
}

fn main() {
    let mut input = String::new();
    println!("Enter your code:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let words: Vec<&str> = input.split_whitespace().collect();
    let empty_list_parser: Vec<&str> = Vec::new();
    let empty_list: Vec<(&str, Tipo)> = Vec::new();

    let (term_parser, vec_string_parser) = pass_to_term(words);

    if vec_string_parser != empty_list_parser {
        panic!("!");
    } else {
        let typing = type_values(term_parser, empty_list);
        print_result(&typing);
    }
}