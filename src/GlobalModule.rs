use crate::di_symbols::DI;

const $config = {
    provide: DI.config,
    useValue: load_config(),
};

const $db = {
    provide: DI.db,
    useFactory: fn,
    inject: [ DI.config ],
};