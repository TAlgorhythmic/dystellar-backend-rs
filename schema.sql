CREATE TABLE IF NOT EXISTS players
(
    uuid CHAR(36) NOT NULL,
    chat BOOLEAN NOT NULL,
    messages INTEGER NOT NULL DEFAULT 1,
    suffix VARCHAR(15) NOT NULL,
    punishments VARCHAR(1000),
    notes VARCHAR(1000),
    lang VARCHAR(4) NOT NULL,
    inbox VARCHAR(3000),
    tabcompletion BOOLEAN NOT NULL DEFAULT FALSE,
    scoreboard BOOLEAN NOT NULL DEFAULT TRUE,
    ignoreList VARCHAR(4000) NOT NULL,
    friends VARCHAR(2000),
    otherConfigs VARCHAR(4000),
    tips VARCHAR(50) NOT NULL,
    PRIMARY KEY (uuid)
);
CREATE TABLE IF NOT EXISTS senders
(
    id INTEGER NOT NULL,
    serialized VARCHAR(4000) NOT NULL,
	PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS punishments
(
	id INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY,
	reason VARCHAR(255) NOT NULL DEFAULT "Unspecified",
	creationDate DATETIME NOT NULL,
	expirationDate DATETIME,
	type INTEGER NOT NULL,
	player CHAR(36) NOT NULL,
	FOREIGN KEY (player) REFERENCES players(uuid)
);
