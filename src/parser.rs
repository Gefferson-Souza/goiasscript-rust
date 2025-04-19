use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use crate::errors::CompileError;
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        Ok(Program { statements })
    }

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_tokens(&[TokenKind::Uai]) {
            self.var_declaration(false)
        } else if self.match_tokens(&[TokenKind::Trem]) {
            self.var_declaration(true)
        } else if self.match_tokens(&[TokenKind::PrestaServico]) {
            self.function_declaration(false)
        } else if self.match_tokens(&[TokenKind::VaiNaFrente]) {
            if self.check(TokenKind::PrestaServico) {
                self.advance(); // Consume PrestaServico
                self.function_declaration(true)
            } else {
                Err(self.error("Esperava 'presta_serviço' após 'vai_na_frente'"))
            }
        } else if self.match_tokens(&[TokenKind::ArrumaTrem]) {
            // Suporte para classes
            self.class_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self, is_mutable: bool) -> Result<Statement> {
        // Verifica se a próxima sequência é a palavra-chave 'ocê' seguida por um ponto
        if self.check(TokenKind::Oce) && self.check_next(TokenKind::Dot) {
            // Esta é uma atribuição de propriedade, não uma declaração de variável comum
            return self.property_assignment(is_mutable);
        }

        // Continua com a declaração de variável normal
        let name = self.consume(TokenKind::Identifier, "Esperava nome de variável")?;
        
        let initializer = if self.match_tokens(&[TokenKind::E]) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Semicolon, "Esperava ';' após declaração de variável")?;
        
        Ok(Statement::VarDeclaration {
            is_mutable,
            name: name.lexeme,
            initializer,
        })
    }

    // Novo método para lidar com atribuição de propriedades
    fn property_assignment(&mut self, _is_mutable: bool) -> Result<Statement> {
        // Consumir a palavra-chave 'ocê'
        self.advance(); // consome 'ocê'
        self.advance(); // consome o ponto '.'
        
        let property = self.consume(TokenKind::Identifier, "Esperava nome de propriedade após 'ocê.'")?;
        
        self.consume(TokenKind::E, "Esperava 'é' após nome de propriedade")?;
        
        let value = self.expression()?;
        
        self.consume(TokenKind::Semicolon, "Esperava ';' após atribuição de propriedade")?;
        
        Ok(Statement::PropertyAssignment {
            object: Expression::This,
            property: property.lexeme,
            value,
        })
    }

    // Método auxiliar para verificar o próximo token após o atual
    fn check_next(&self, kind: TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current + 1].kind) == std::mem::discriminant(&kind)
        }
    }

    fn function_declaration(&mut self, is_async: bool) -> Result<Statement> {
        let name = self.consume(TokenKind::Identifier, "Esperava nome de função")?;
        
        self.consume(TokenKind::OpenParen, "Esperava '(' após nome de função")?;
        let mut params = Vec::new();
        
        if !self.check(TokenKind::CloseParen) {
            // Corrigindo o erro com "do"
            loop {
                params.push(self.consume(TokenKind::Identifier, "Esperava nome de parâmetro")?.lexeme);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::CloseParen, "Esperava ')' após parâmetros")?;
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do corpo da função")?;
        
        let body = self.block()?;
        
        Ok(Statement::FunctionDeclaration {
            name: name.lexeme,
            params,
            body,
            is_async,
        })
    }

    fn class_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenKind::Identifier, "Esperava nome da classe após 'arruma_trem'")?;
        
        // Verifica se tem herança
        let parent = if self.match_tokens(&[TokenKind::InherdaDe, TokenKind::ETipoDe]) {
            Some(self.consume(TokenKind::Identifier, "Esperava nome da classe pai")?.lexeme)
        } else {
            None
        };
        
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do corpo da classe")?;
        
        let mut methods = Vec::new();
        
        while !self.check(TokenKind::CloseBrace) && !self.is_at_end() {
            let is_constructor = self.match_tokens(&[TokenKind::ApreparaTrem]);
            let is_static = self.match_tokens(&[TokenKind::NumMuda]);
            let is_async = self.match_tokens(&[TokenKind::VaiNaFrente]);
            
            // Assume nome do método ou construtor
            let method_name = if is_constructor {
                "constructor".to_string()
            } else {
                self.consume(TokenKind::Identifier, "Esperava nome do método")?.lexeme
            };
            
            self.consume(TokenKind::OpenParen, "Esperava '(' após nome do método")?;
            
            // Parâmetros do método
            let mut params = Vec::new();
            if !self.check(TokenKind::CloseParen) {
                loop {
                    params.push(self.consume(TokenKind::Identifier, "Esperava nome de parâmetro")?.lexeme);
                    
                    if !self.match_tokens(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(TokenKind::CloseParen, "Esperava ')' após parâmetros")?;
            self.consume(TokenKind::OpenBrace, "Esperava '{' antes do corpo do método")?;
            
            let body = self.block()?;
            
            methods.push(ClassMethod {
                name: method_name,
                params,
                body,
                is_static,
                is_constructor,
                is_async,
            });
        }
        
        self.consume(TokenKind::CloseBrace, "Esperava '}' após corpo da classe")?;
        
        Ok(Statement::ClassDeclaration {
            name: name.lexeme,
            methods,
            parent,
        })
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.match_tokens(&[TokenKind::SeOceQuiser]) {
            self.if_statement()
        } else if self.match_tokens(&[TokenKind::EnquantoTiver]) {
            self.while_statement()
        } else if self.match_tokens(&[TokenKind::VaiIndo]) {
            self.for_statement()
        } else if self.match_tokens(&[TokenKind::FazFavor]) {
            self.return_statement()
        } else if self.match_tokens(&[TokenKind::OpenBrace]) {
            Ok(Statement::Block { statements: self.block()? })
        } else if self.match_tokens(&[TokenKind::TentaAi]) {
            self.try_statement()
        } else if self.match_tokens(&[TokenKind::ParaComIsso]) {
            self.consume(TokenKind::Semicolon, "Esperava ';' após 'para_com_isso'")?;
            Ok(Statement::BreakStatement)
        } else if self.match_tokens(&[TokenKind::ContinuaAi]) {
            self.consume(TokenKind::Semicolon, "Esperava ';' após 'continua_aí'")?;
            Ok(Statement::ContinueStatement)
        } else {
            self.expression_statement()
        }
    }
    
    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::OpenParen, "Esperava '(' após 'se_ocê_quiser'")?;
        let condition = self.expression()?;
        self.consume(TokenKind::CloseParen, "Esperava ')' após condição")?;
        
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do bloco if")?;
        let then_branch = self.block()?;
        
        let mut else_branch = None;
        if self.match_tokens(&[TokenKind::SeNao]) {
            self.consume(TokenKind::OpenBrace, "Esperava '{' antes do bloco else")?;
            else_branch = Some(self.block()?);
        }
        
        Ok(Statement::IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::OpenParen, "Esperava '(' após 'enquanto_tiver'")?;
        let condition = self.expression()?;
        self.consume(TokenKind::CloseParen, "Esperava ')' após condição")?;
        
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do corpo do loop")?;
        let body = self.block()?;
        
        Ok(Statement::WhileStatement {
            condition,
            body,
        })
    }
    
    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::OpenParen, "Esperava '(' após 'vai_indo'")?;
        
        let initializer = if self.match_tokens(&[TokenKind::Semicolon]) {
            None
        } else if self.match_tokens(&[TokenKind::Uai]) {
            Some(Box::new(self.var_declaration(false)?))
        } else if self.match_tokens(&[TokenKind::Trem]) {
            Some(Box::new(self.var_declaration(true)?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };
        
        let condition = if !self.check(TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::Semicolon, "Esperava ';' após condição do loop")?;
        
        let increment = if !self.check(TokenKind::CloseParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::CloseParen, "Esperava ')' após cláusulas do for")?;
        
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do corpo do loop")?;
        let body = self.block()?;
        
        Ok(Statement::ForStatement {
            initializer,
            condition,
            increment,
            body,
        })
    }
    
    fn return_statement(&mut self) -> Result<Statement> {
        let value = if !self.check(TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Semicolon, "Esperava ';' após retorno")?;
        
        Ok(Statement::ReturnStatement {
            value,
        })
    }
    
    fn try_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::OpenBrace, "Esperava '{' após 'tenta_aí'")?;
        let try_block = self.block()?;
        
        self.consume(TokenKind::SeDerRuim, "Esperava 'se_der_ruim' após bloco try")?;
        self.consume(TokenKind::OpenParen, "Esperava '(' após 'se_der_ruim'")?;
        let catch_var = self.consume(TokenKind::Identifier, "Esperava nome de variável para erro")?.lexeme;
        self.consume(TokenKind::CloseParen, "Esperava ')' após variável de erro")?;
        
        self.consume(TokenKind::OpenBrace, "Esperava '{' antes do bloco catch")?;
        let catch_block = self.block()?;
        
        Ok(Statement::TryStatement {
            try_block,
            catch_var,
            catch_block,
        })
    }
    
    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Esperava ';' após expressão")?;
        
        Ok(Statement::ExpressionStatement {
            expression: expr,
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        while !self.check(TokenKind::CloseBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        self.consume(TokenKind::CloseBrace, "Esperava '}' após bloco")?;
        
        Ok(statements)
    }
    
    fn expression(&mut self) -> Result<Expression> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expression> {
        let mut expr = self.logic_and()?;
        
        while self.match_tokens(&[TokenKind::OuEntao]) {
            let operator = BinaryOperator::Or;
            let right = self.logic_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logic_and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;
        
        while self.match_tokens(&[TokenKind::EMais]) {
            let operator = BinaryOperator::And;
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;
        
        while self.match_tokens(&[TokenKind::EIgualim, TokenKind::Diferente]) {
            let operator = match self.previous().kind {
                TokenKind::EIgualim => BinaryOperator::Equal,
                TokenKind::Diferente => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;
        
        while self.match_tokens(&[
            TokenKind::MaiorQue, TokenKind::MenorQue, 
            TokenKind::PeloMenos, TokenKind::NoMaximo
        ]) {
            let operator = match self.previous().kind {
                TokenKind::MaiorQue => BinaryOperator::Greater,
                TokenKind::MenorQue => BinaryOperator::Less,
                TokenKind::PeloMenos => BinaryOperator::GreaterEqual,
                TokenKind::NoMaximo => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;
        
        while self.match_tokens(&[TokenKind::Mais, TokenKind::Menos]) {
            let operator = match self.previous().kind {
                TokenKind::Mais => BinaryOperator::Add,
                TokenKind::Menos => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.call()?;
        
        while self.match_tokens(&[TokenKind::Vezes, TokenKind::Dividido, TokenKind::Sobrou]) {
            let operator = match self.previous().kind {
                TokenKind::Vezes => BinaryOperator::Multiply,
                TokenKind::Dividido => BinaryOperator::Divide,
                TokenKind::Sobrou => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = self.call()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_tokens(&[TokenKind::OpenParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_tokens(&[TokenKind::Dot]) {
                let name = self.consume(TokenKind::Identifier, "Esperava nome de propriedade após '.'")?;
                expr = Expression::GetProperty {
                    object: Box::new(expr),
                    property: name.lexeme,
                };
            } else if self.match_tokens(&[TokenKind::OpenBracket]) {
                let index = self.expression()?;
                self.consume(TokenKind::CloseBracket, "Esperava ']' após índice")?;
                
                expr = Expression::Call {
                    callee: Box::new(Expression::GetProperty {
                        object: Box::new(expr),
                        property: "get_index".to_string(),
                    }),
                    arguments: vec![index],
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression> {
        // Adiciona reconhecimento para 'oce' (this) antes dos outros identificadores
        if self.match_tokens(&[TokenKind::Oce]) {
            return Ok(Expression::This);
        }

        if self.match_tokens(&[TokenKind::StringLiteral]) {
            let value = self.previous().lexeme.clone();
            let value = if value.len() >= 2 {
                value[1..value.len()-1].to_string()
            } else {
                "".to_string()
            };
            
            return Ok(Expression::Literal { 
                value: LiteralValue::String(value) 
            });
        } 
        
        if self.match_tokens(&[TokenKind::NumberLiteral]) {
            let value = self.previous().lexeme.parse::<f64>().unwrap_or(0.0);
            return Ok(Expression::Literal { 
                value: LiteralValue::Number(value) 
            });
        } 
        
        if self.match_tokens(&[TokenKind::Certeza]) {
            return Ok(Expression::Literal { 
                value: LiteralValue::Bool(true) 
            });
        } 
        
        if self.match_tokens(&[TokenKind::DeJeitoNenhum]) {
            return Ok(Expression::Literal { 
                value: LiteralValue::Bool(false) 
            });
        } 
        
        if self.match_tokens(&[TokenKind::Vazio]) {
            return Ok(Expression::Literal { 
                value: LiteralValue::Null 
            });
        } 
        
        if self.match_tokens(&[TokenKind::Identifier]) {
            let name = self.previous().lexeme.clone();
            
            if self.check(TokenKind::E) {
                self.advance();
                let value = self.expression()?;
                return Ok(Expression::Assignment {
                    name,
                    value: Box::new(value)
                });
            }
            
            return Ok(Expression::Variable { name });
        }
        
        if self.match_tokens(&[TokenKind::Prosa, TokenKind::Reclama]) {
            return Ok(Expression::Variable { 
                name: self.previous().lexeme.clone() 
            });
        }
        
        if self.match_tokens(&[TokenKind::OpenParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::CloseParen, "Esperava ')' após expressão")?;
            return Ok(expr);
        }

        if self.match_tokens(&[TokenKind::OpenBracket]) {
            return self.array_literal();
        }
        
        if self.match_tokens(&[TokenKind::OpenBrace]) {
            return self.object_literal();
        }

        if self.match_tokens(&[TokenKind::FazUm]) {
            let class_name = self.consume(TokenKind::Identifier, "Esperava nome de classe após 'faz_um'")?;
            self.consume(TokenKind::OpenParen, "Esperava '(' após nome de classe")?;
            
            let mut arguments = Vec::new();
            if !self.check(TokenKind::CloseParen) {
                loop {
                    arguments.push(self.expression()?);
                    if !self.match_tokens(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(TokenKind::CloseParen, "Esperava ')' após argumentos")?;
            
            return Ok(Expression::New {
                class: Box::new(Expression::Variable { name: class_name.lexeme }),
                arguments,
            });
        }
        
        Err(self.error("Expressão esperada"))
    }

    fn array_literal(&mut self) -> Result<Expression> {
        let mut elements = Vec::new();

        if !self.check(TokenKind::CloseBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenKind::CloseBracket, "Esperava ']' após elementos do array")?;

        Ok(Expression::Array { elements })
    }

    fn object_literal(&mut self) -> Result<Expression> {
        let mut properties = Vec::new();

        if self.match_tokens(&[TokenKind::CloseBrace]) {
            return Ok(Expression::Object { properties });
        }

        loop {
            let key = self.consume(TokenKind::Identifier, "Esperava nome de propriedade")?;
            self.consume(TokenKind::Colon, "Esperava ':' após nome da propriedade")?;
            let value = self.expression()?;
            properties.push((key.lexeme, value));
            
            if !self.match_tokens(&[TokenKind::Comma]) {
                break;
            }
        }

        self.consume(TokenKind::CloseBrace, "Esperava '}' após propriedades do objeto")?;

        Ok(Expression::Object { properties })
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression> {
        let mut arguments = Vec::new();
        
        if !self.check(TokenKind::CloseParen) {
            loop {
                arguments.push(self.expression()?);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::CloseParen, "Esperava ')' após argumentos da função")?;
        
        Ok(Expression::Call { 
            callee: Box::new(callee), 
            arguments 
        })
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token> {
        if self.check(kind.clone()) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn error(&self, message: &str) -> anyhow::Error {
        let span = if !self.is_at_end() {
            let current_token = self.peek();
            let current_span = current_token.span.clone();
            
            // Adiciona informações de depuração para qualquer erro
            eprintln!("DEBUG: Erro no token: '{}' (tipo: {:?}, posição: {:?})", 
                     current_token.lexeme, current_token.kind, current_span);
            
            if self.current > 0 {
                eprintln!("DEBUG: Token anterior: '{}' (tipo: {:?})", 
                         self.previous().lexeme, self.previous().kind);
            }
            
            if self.current + 1 < self.tokens.len() {
                eprintln!("DEBUG: Próximo token: '{}' (tipo: {:?})", 
                         self.tokens[self.current + 1].lexeme, self.tokens[self.current + 1].kind);
            }
            
            current_span
        } else if !self.tokens.is_empty() {
            self.tokens.last().unwrap().span.clone()
        } else {
            0..0
        };
        
        CompileError::ParserError { span, message: message.to_string() }.into()
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}