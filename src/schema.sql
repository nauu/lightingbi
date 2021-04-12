CREATE TABLE IF NOT EXISTS t_user (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` text DEFAULT NULL,
    `age` int,
    PRIMARY KEY (`id`)
)ENGINE=InnoDB;
