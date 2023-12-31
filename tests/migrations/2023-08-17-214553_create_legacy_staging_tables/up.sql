CREATE TABLE `staging_customer` (
  `id_source_client` int(11) NOT NULL,
  `id_source_contact` int(11) NOT NULL,
  `id` int(11) DEFAULT NULL,
  `id_shop` int(11) NOT NULL,
  `m_pricelist_id` int(11) NOT NULL,
  `name` varchar(255) NOT NULL,
  `company` varchar(255) DEFAULT NULL,
  `email` varchar(128) NOT NULL,
  `active` bit(1) NOT NULL,
  `is_xxa_centrale` bit(1) NOT NULL,
  `free_shipping_amount` int(11) NOT NULL,
  `update_client` datetime NOT NULL,
  `update_contact` datetime NOT NULL,
  `is_synchronised` bit(1) NOT NULL,
  `has_error` bit(1) NOT NULL,
  `force_update` bit(1) NOT NULL DEFAULT b'0',
  PRIMARY KEY (`id_source_contact`),
  UNIQUE KEY `email` (`email`),
  UNIQUE KEY `id_source_contact` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `language_list` (
  `locale` varchar(5) NOT NULL,
  `id` int(11) NOT NULL,
  PRIMARY KEY (`locale`),
  UNIQUE KEY `id_target_language` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;