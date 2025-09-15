CREATE TABLE IF NOT EXISTS message_tree (
    branch INTEGER NOT NULL,
    leaf INTEGER NOT NULL,
    depth INTEGER NOT NULL,

    PRIMARY KEY (branch, leaf),
    FOREIGN KEY(branch) REFERENCES message(id),
    FOREIGN KEY(leaf) REFERENCES message(id)
);
