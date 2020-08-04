use crate::{
    Arena, Constant, Expression, FastHashMap, Function, GlobalVariable, Handle, LocalVariable,
    ShaderStage, Statement, Type,
};

#[derive(Debug)]
pub struct Program {
    pub version: u16,
    pub profile: Profile,
    pub shader_stage: ShaderStage,
    pub lookup_function: FastHashMap<String, Handle<Function>>,
    pub functions: Arena<Function>,
    pub lookup_type: FastHashMap<String, Handle<Type>>,
    pub types: Arena<Type>,
    pub constants: Arena<Constant>,
    pub global_variables: Arena<GlobalVariable>,
    pub context: Context,
}

impl Program {
    pub fn new(shader_stage: ShaderStage) -> Program {
        Program {
            version: 0,
            profile: Profile::Core,
            shader_stage,
            lookup_function: FastHashMap::default(),
            functions: Arena::<Function>::new(),
            lookup_type: FastHashMap::default(),
            types: Arena::<Type>::new(),
            constants: Arena::<Constant>::new(),
            global_variables: Arena::<GlobalVariable>::new(),
            context: Context {
                expressions: Arena::<Expression>::new(),
                local_variables: Arena::<LocalVariable>::new(),
            },
        }
    }
}

#[derive(Debug)]
pub enum Profile {
    Core,
}

#[derive(Debug)]
pub struct Context {
    pub expressions: Arena<Expression>,
    pub local_variables: Arena<LocalVariable>,
}

#[derive(Debug)]
pub struct ExpressionRule {
    pub expression: Handle<Expression>,
    pub statements: Vec<Statement>,
}