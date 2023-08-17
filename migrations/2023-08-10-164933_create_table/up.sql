-- MySQL dump 10.13  Distrib 8.0.33, for macos13.3 (arm64)
--
-- Host: 127.0.0.1    Database: poolweb
-- ------------------------------------------------------
-- Server version	11.0.2-MariaDB-1:11.0.2+maria~ubu2204

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `mapping_client_contact`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `mapping_client_contact` (
  `idp_id_client` int(10) unsigned NOT NULL,
  `ps_id_customer` int(10) unsigned NOT NULL,
  PRIMARY KEY (`idp_id_client`),
  KEY `IDX_AF893667C860CCC5` (`ps_id_customer`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `order`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `order` (
  `id_order` int(10) unsigned NOT NULL,
  `id_client` int(10) unsigned NOT NULL,
  `order_ref` varchar(32) NOT NULL,
  `date` datetime NOT NULL,
  `order_status` varchar(128) DEFAULT NULL,
  `delivery_status` varchar(128) DEFAULT NULL,
  PRIMARY KEY (`id_order`),
  UNIQUE KEY `UNIQ_F5299398573471C3` (`order_ref`),
  KEY `IDX_F5299398E173B1B8` (`id_client`),
  CONSTRAINT `FK_F5299398E173B1B8` FOREIGN KEY (`id_client`) REFERENCES `mapping_client_contact` (`idp_id_client`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `order_line`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `order_line` (
  `id_order_line` int(10) unsigned NOT NULL,
  `id_order` int(10) unsigned NOT NULL,
  `order_ref` varchar(32) NOT NULL,
  `po_ref` varchar(32) NOT NULL,
  `product_ref` varchar(32) NOT NULL,
  `product_name` varchar(32) DEFAULT NULL,
  `qty_ordered` int(10) unsigned NOT NULL,
  `qty_reserved` int(10) unsigned NOT NULL,
  `qty_delivered` int(10) unsigned NOT NULL,
  `due_date` datetime NOT NULL,
  PRIMARY KEY (`id_order_line`),
  KEY `IDX_9CE58EE11BACD2A8` (`id_order`),
  CONSTRAINT `FK_9CE58EE11BACD2A8` FOREIGN KEY (`id_order`) REFERENCES `order` (`id_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-08-10 19:40:52

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