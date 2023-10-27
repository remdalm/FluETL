/* Product is only for testing the many to many self referencing relationship */
CREATE TABLE `product` (
  `id_product` int(10) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id_product`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/* Populate it for test purposes*/
INSERT INTO `product` (`id_product`) VALUES (1),(2),(3),(4),(5),(6),(7),(8),(9),(10);

CREATE TABLE `product_substitute` (
  `id_product` int(10) unsigned NOT NULL,
  `id_substitute` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id_product`,`id_substitute`),
  KEY `IDX_5C940C92DD7ADDD` (`id_product`),
  KEY `IDX_5C940C926A79D36E` (`id_substitute`),
  CONSTRAINT `FK_5C940C926A79D36E` FOREIGN KEY (`id_substitute`) REFERENCES `product` (`id_product`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `FK_5C940C92DD7ADDD` FOREIGN KEY (`id_product`) REFERENCES `product` (`id_product`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;