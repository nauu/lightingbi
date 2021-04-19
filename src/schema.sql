CREATE TABLE IF NOT EXISTS t_lighting_user (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` text DEFAULT NULL,
    `age` int,
    PRIMARY KEY (`id`)
)ENGINE=InnoDB;


CREATE TABLE IF NOT EXISTS t_lighting_dataset (
    `id` varchar(128) NOT NULL ,
    `name` varchar(128) NOT NULL,
    `display_name` varchar(128),
    `engine_type` varchar(128),
    `size` double,
    `count` int,
    PRIMARY KEY (`id`)
)ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS t_lighting_field (
    `id` varchar(128) NOT NULL ,
    `name` varchar(128) NOT NULL,
    `dataset_id` varchar(128) NOT NULL,
    `data_type` varchar(128) NOT NULL,
    `field_type` varchar(128) NOT NULL,
    `display_name` varchar(128),
    `formula` varchar(128),
    PRIMARY KEY (`id`)
)ENGINE=InnoDB;