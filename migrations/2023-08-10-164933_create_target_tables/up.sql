/** MAPPING CLIENT */
DROP TABLE IF EXISTS `mapping_client_contact`;
CREATE TABLE `mapping_client_contact`
(
    `id_customer`   int(10) unsigned NOT NULL,
    `idp_id_client` int(10) unsigned NOT NULL,
    PRIMARY KEY (`id_customer`),
    KEY `IDX_AF8936671C01EB63` (`idp_id_client`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

/** ORDER */
DROP TABLE IF EXISTS `order`;
CREATE TABLE `order`
(
    `id_order`     int(10) unsigned NOT NULL,
    `id_client`    int(10) unsigned NOT NULL,
    `client_name`  varchar(255)     DEFAULT NULL,
    `order_ref`    varchar(32)      NOT NULL,
    `date`         datetime         NOT NULL,
    `po_ref`       varchar(255)     DEFAULT NULL,
    `origin`       varchar(255)     DEFAULT NULL,
    `completion`   int(10) unsigned DEFAULT NULL,
    `order_status` varchar(2)       DEFAULT NULL,
    PRIMARY KEY (`id_order`),
    UNIQUE KEY `UNIQ_F5299398573471C3` (`order_ref`),
    KEY `IDX_F5299398E173B1B8` (`id_client`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

/** ORDER LINE */
DROP TABLE IF EXISTS `order_line`;
CREATE TABLE `order_line`
(
    `id_order_line` int(10) unsigned NOT NULL,
    `id_order`      int(10) unsigned NOT NULL,
    `product_ref`   varchar(64)      NOT NULL,
    `qty_ordered`   int(10) unsigned NOT NULL,
    `qty_reserved`  int(10) unsigned NOT NULL,
    `qty_delivered` int(10) unsigned NOT NULL,
    `due_date`      date DEFAULT NULL,
    PRIMARY KEY (`id_order_line`),
    KEY `IDX_9CE58EE11BACD2A8` (`id_order`),
    CONSTRAINT `FK_9CE58EE11BACD2A8` FOREIGN KEY (`id_order`) REFERENCES `order` (`id_order`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

CREATE TABLE `order_line_lang`
(
    `id_order_line` int(10) unsigned NOT NULL,
    `id_lang`       int(10) unsigned NOT NULL,
    `product_name`  varchar(255)     NOT NULL,
    PRIMARY KEY (`id_order_line`, `id_lang`),
    KEY `IDX_6C30C43396F0E36` (`id_order_line`),
    KEY `IDX_6C30C43BA299860` (`id_lang`),
    CONSTRAINT `FK_6C30C43396F0E36` FOREIGN KEY (`id_order_line`) REFERENCES `order_line` (`id_order_line`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

/** DELIVERY SLIP */
DROP TABLE IF EXISTS `delivery_slip`;
CREATE TABLE `delivery_slip`
(
    `id_delivery_slip` int(10) unsigned NOT NULL,
    `id_client`        int(10) unsigned NOT NULL,
    `reference`        varchar(32)      NOT NULL,
    `shipping_date`    date         DEFAULT NULL,
    `po_ref`           varchar(255) DEFAULT NULL,
    `carrier_name`     varchar(255) DEFAULT NULL,
    `status`           varchar(128) DEFAULT NULL,
    `tracking_number`  varchar(255) DEFAULT NULL,
    `tracking_link`    varchar(255) DEFAULT NULL,
    PRIMARY KEY (`id_delivery_slip`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

/** INVOICE */
DROP TABLE IF EXISTS `invoice`;
CREATE TABLE `invoice` (
  `id_invoice` int(10) unsigned NOT NULL,
  `id_client` int(10) unsigned NOT NULL,
  `client_name` varchar(255) DEFAULT NULL,
  `invoice_ref` varchar(32) NOT NULL,
  `date` date NOT NULL,
  `file_name` varchar(255) DEFAULT NULL,
  `po_ref` varchar(255) DEFAULT NULL,
  `total_tax_excl` decimal(10,2) NOT NULL,
  `total_tax_incl` decimal(10,2) NOT NULL,
  PRIMARY KEY (`id_invoice`),
  KEY `IDX_90651744E173B1B8` (`id_client`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


DROP TABLE IF EXISTS `invoice_lang`;
CREATE TABLE `invoice_lang` (
  `id_invoice` int(10) unsigned NOT NULL,
  `id_lang` int(10) unsigned NOT NULL,
  `type_name` varchar(255) NOT NULL,
  PRIMARY KEY (`id_invoice`,`id_lang`),
  KEY `IDX_33CCE6074EF8BE34` (`id_invoice`),
  KEY `IDX_33CCE607BA299860` (`id_lang`),
  CONSTRAINT `FK_33CCE6074EF8BE34` FOREIGN KEY (`id_invoice`) REFERENCES `invoice` (`id_invoice`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;