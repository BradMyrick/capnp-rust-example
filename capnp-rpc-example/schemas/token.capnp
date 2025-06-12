@0xe6c84142c1958685;

interface Root {
    getAuth @0 () -> (auth :Auth);
}

interface Auth {
    login @0 (password :Text) -> (token :Token);
}

interface Token {
    mint @0 () -> (result :Text);
}

