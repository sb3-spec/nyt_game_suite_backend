CREATE TABLE valid_words(
    id serial unique,
    word text not null
);

CREATE TABLE word_bank(
    id serial unique,
    word text not null,
    last_used_on date
);
