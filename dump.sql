-- MySQL dump 10.13  Distrib 8.0.37, for Linux (x86_64)
--
-- Host: localhost    Database: work_experience
-- ------------------------------------------------------
-- Server version	8.0.37-0ubuntu0.22.04.3

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
-- Position to start replication or point-in-time recovery from
--

-- CHANGE MASTER TO MASTER_LOG_FILE='binlog.000021', MASTER_LOG_POS=157;

--
-- Current Database: `work_experience`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `work_experience` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `work_experience`;

--
-- Table structure for table `auth`
--

DROP TABLE IF EXISTS `auth`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `auth` (
  `id` int NOT NULL AUTO_INCREMENT,
  `login_id` varchar(100) NOT NULL,
  `salt` varchar(100) DEFAULT NULL,
  `hash` varchar(100) DEFAULT NULL,
  `expire_date` date DEFAULT NULL,
  `comment` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `login_id` (`login_id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `auth`
--

LOCK TABLES `auth` WRITE;
/*!40000 ALTER TABLE `auth` DISABLE KEYS */;
INSERT INTO `auth` VALUES (6,'yonetani','zWAb3Qh3tA','zWAb3Qh3tA5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8','2024-09-30',NULL),(7,'tanaka','wvmytE6wA2','wvmytE6wA25e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8','2024-09-30',NULL),(8,'test','g5uVJ5Va0V','g5uVJ5Va0V9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08','2024-10-30',NULL);
/*!40000 ALTER TABLE `auth` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `departments`
--

DROP TABLE IF EXISTS `departments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `departments` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(20) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `departments`
--

LOCK TABLES `departments` WRITE;
/*!40000 ALTER TABLE `departments` DISABLE KEYS */;
INSERT INTO `departments` VALUES (1,'営業'),(2,'総務');
/*!40000 ALTER TABLE `departments` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `dev_tool`
--

DROP TABLE IF EXISTS `dev_tool`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `dev_tool` (
  `id` int NOT NULL AUTO_INCREMENT,
  `project_no` int DEFAULT NULL,
  `dev_tool_type1` varchar(100) DEFAULT NULL,
  `dev_tool_name1` varchar(100) DEFAULT NULL,
  `dev_tool_type2` varchar(100) DEFAULT NULL,
  `dev_tool_name2` varchar(100) DEFAULT NULL,
  `dev_tool_type3` varchar(100) DEFAULT NULL,
  `dev_tool_name3` varchar(100) DEFAULT NULL,
  `dev_tool_type4` varchar(100) DEFAULT NULL,
  `dev_tool_name4` varchar(100) DEFAULT NULL,
  `dev_tool_type5` varchar(100) DEFAULT NULL,
  `dev_tool_name5` varchar(100) DEFAULT NULL,
  `dev_tool_type6` varchar(100) DEFAULT NULL,
  `dev_tool_name6` varchar(100) DEFAULT NULL,
  `dev_tool_type7` varchar(100) DEFAULT NULL,
  `dev_tool_name7` varchar(100) DEFAULT NULL,
  `dev_tool_type8` varchar(100) DEFAULT NULL,
  `dev_tool_name8` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `project_no` (`project_no`),
  CONSTRAINT `dev_tool_ibfk_1` FOREIGN KEY (`project_no`) REFERENCES `work_experience` (`project_no`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `dev_tool`
--

LOCK TABLES `dev_tool` WRITE;
/*!40000 ALTER TABLE `dev_tool` DISABLE KEYS */;
INSERT INTO `dev_tool` VALUES (1,1,'インフラ','AWS（lambda、ECS、SQS、APIgateway、CodeCommit）','DB','AWS(DynamoDB)','デプロイ','AWS （CodePipeline、CodeCommit','調査ツール','AWS（CloudWatch、Athena）','使用OS','Win10、Ubuntu',NULL,NULL,NULL,NULL,NULL,NULL),(3,2,'インフラ','AWS（S3、lambda)','DB','AWS（RDS（postgreSQL,））',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(4,3,'インフラ','Docker','フレームワーク','Nuxt.js',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(5,4,'インフラ','Docker','フレームワーク','cakePHP','DB','OracleDB',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(6,5,'デザインツール','Adobe Photoshop',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(7,6,'フレームワーク','Laravel','DB','MySQL','インフラ','Docker',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(8,7,'フレームワーク','Laravel','DB','MySQL','インフラ','Docker',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(9,8,'フレームワーク','損保会社独自制作のFW','DB','OracleDB',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
/*!40000 ALTER TABLE `dev_tool` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `dev_tool_mst`
--

DROP TABLE IF EXISTS `dev_tool_mst`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `dev_tool_mst` (
  `id` int NOT NULL AUTO_INCREMENT,
  `dev_tool_type` varchar(50) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `dev_tool_mst`
--

LOCK TABLES `dev_tool_mst` WRITE;
/*!40000 ALTER TABLE `dev_tool_mst` DISABLE KEYS */;
INSERT INTO `dev_tool_mst` VALUES (1,'インフラ'),(2,'DB'),(3,'デプロイ'),(4,'調査ツール'),(5,'使用OS');
/*!40000 ALTER TABLE `dev_tool_mst` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `employees`
--

DROP TABLE IF EXISTS `employees`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `employees` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(20) DEFAULT NULL,
  `department_id` int DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `department_id` (`department_id`),
  CONSTRAINT `employees_ibfk_1` FOREIGN KEY (`department_id`) REFERENCES `departments` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `employees`
--

LOCK TABLES `employees` WRITE;
/*!40000 ALTER TABLE `employees` DISABLE KEYS */;
INSERT INTO `employees` VALUES (1,'田中',1),(2,'佐々木',2),(5,'鈴木',1);
/*!40000 ALTER TABLE `employees` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `pjt_support_tool`
--

DROP TABLE IF EXISTS `pjt_support_tool`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `pjt_support_tool` (
  `id` int NOT NULL AUTO_INCREMENT,
  `project_no` int DEFAULT NULL,
  `pjt_support_tool_type1` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name1` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type2` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name2` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type3` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name3` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type4` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name4` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type5` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name5` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type6` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name6` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type7` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name7` varchar(100) DEFAULT NULL,
  `pjt_support_tool_type8` varchar(100) DEFAULT NULL,
  `pjt_support_tool_name8` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `project_no` (`project_no`),
  CONSTRAINT `pjt_support_tool_ibfk_1` FOREIGN KEY (`project_no`) REFERENCES `work_experience` (`project_no`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `pjt_support_tool`
--

LOCK TABLES `pjt_support_tool` WRITE;
/*!40000 ALTER TABLE `pjt_support_tool` DISABLE KEYS */;
INSERT INTO `pjt_support_tool` VALUES (1,1,'タスク管理ツール','Jira','ドキュメント管理','Confluence',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(3,2,'タスク管理ツール','RedMain','','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(4,3,'タスク管理ツール','RedMain','','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(5,4,'タスク管理ツール','RedMain','','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(6,5,'タスク管理ツール','Backlog',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(7,6,'タスク管理ツール','RedMain',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(8,7,'タスク管理ツール','Trello',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL),(9,8,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
/*!40000 ALTER TABLE `pjt_support_tool` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `pjt_support_tool_mst`
--

DROP TABLE IF EXISTS `pjt_support_tool_mst`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `pjt_support_tool_mst` (
  `id` int NOT NULL AUTO_INCREMENT,
  `pjt_support_tool_type` varchar(50) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `pjt_support_tool_mst`
--

LOCK TABLES `pjt_support_tool_mst` WRITE;
/*!40000 ALTER TABLE `pjt_support_tool_mst` DISABLE KEYS */;
INSERT INTO `pjt_support_tool_mst` VALUES (1,'タスク管理ツール'),(2,'ドキュメント管理');
/*!40000 ALTER TABLE `pjt_support_tool_mst` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `qualification`
--

DROP TABLE IF EXISTS `qualification`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `qualification` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) DEFAULT NULL,
  `obtainment_date` date DEFAULT NULL,
  `display` smallint DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `qualification`
--

LOCK TABLES `qualification` WRITE;
/*!40000 ALTER TABLE `qualification` DISABLE KEYS */;
INSERT INTO `qualification` VALUES (1,'基本情報技術者試験 ','2017-10-01',1),(2,'Oracle database MASTER Bronze','2018-12-01',0),(3,'Oracle Java Programmer, Silver','2017-05-01',1);
/*!40000 ALTER TABLE `qualification` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `self_introduction`
--

DROP TABLE IF EXISTS `self_introduction`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `self_introduction` (
  `Id` tinyint NOT NULL,
  `text` text NOT NULL,
  PRIMARY KEY (`Id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `self_introduction`
--

LOCK TABLES `self_introduction` WRITE;
/*!40000 ALTER TABLE `self_introduction` DISABLE KEYS */;
INSERT INTO `self_introduction` VALUES (1,'・不明点や疑問点はまず自分で調べて解決に努め、それでもわからない場合は自己判断せずまわりに協力を仰ぎ、 　正確さを大事にしてこれまで業務に取り組んできました。 ・バックエンド・フロント・インフラなど比較的幅広く何でもできます。');
/*!40000 ALTER TABLE `self_introduction` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `work_experience`
--

DROP TABLE IF EXISTS `work_experience`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `work_experience` (
  `id` int NOT NULL AUTO_INCREMENT,
  `project_no` int NOT NULL,
  `project_title` varchar(100) DEFAULT NULL,
  `member_headcount` int DEFAULT NULL,
  `participate_date` date DEFAULT NULL,
  `leave_date` date DEFAULT NULL,
  `program_language` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  `pjt_content` text,
  `work_kind` varchar(20) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `project_no` (`project_no`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `work_experience`
--

LOCK TABLES `work_experience` WRITE;
/*!40000 ALTER TABLE `work_experience` DISABLE KEYS */;
INSERT INTO `work_experience` VALUES (2,1,'自動車製造会社の車載システムの開発（フルリモート）',7,'2023-08-01','2024-03-30','Java','トヨタの車載システムの保守開発プロジェクトに参画し、設計から試験、本番デプロイまで担当していました。\r\n\r\nアジャイル開発の中の一つのスクラム開発というやり方で開発を行っていました。\r\n\r\nGitリポジトリやデプロイToolなど含め、すべての開発リソースがAWSのプロジェクトで、アクセスログやエラーログの管理もAWSで行ってるプロジェクトでした。\r\n\r\n仕様調査や、稼働中システムでエラー発生の際の調査や、デプロイ時に切り戻しするかの判断など、ソースやアクセスログなどを調べて、仕様や状況を整理して報告するといった運用的なタスクも行っていました。\r\n\r\nチーム内の朝会、夕会のファシリテートも当番制だったので週１でファシリテートもしていました。','基本設計、詳細設計、製造、テスト'),(4,2,'医療研究機関のシステム新規開発（フルリモート）',3,'2023-01-01','2023-06-30','Java（Amazon Corretto）','JavaとAWS（S3、lambda、RDS）を用いたバッチの新規開発。主に詳細設計書の執筆、製造、テストを担当。\r\n　Javaソース、AWSのインフラ資材など何もないところからの完全新規での開発だったので、build.gradleの編集、例外クラスの作成など、DBドライバ接続のコーディングなども行いました。\r\n　また顧客のAWSコンソールを操作させてもらい、S3やRDS等の普段は触らない様な設定を行ったり、Lambda側のメモリを変えながら大きなデータを扱う想定をした性能試験なども行いました。','詳細設計、製造、テスト'),(5,3,'寝具会社の睡眠管理システムの開発（フルリモート）',5,'2022-06-01','2022-12-30','JavaScriot（Vue.js、TypeScript）','Nuxt.js, TypeScriptを使ったフロント開発。\r\n　前任者の交代として単体試験から参画したが、製造のシワ寄せが酷く、ひたすら不具合を修正ししていた。ブランチ開発で一日に何回もコミットするようなスタイルなプロジェクトでした。\r\n　Axiosのレスポンス処理やクッキーの実装など全般的に幅広くフロントエンド開発スキルを伸ばすことができた。\r\n　他にも、バグの調査力、対応方針の提案力も身についた。','詳細設計、製造、テスト'),(6,4,'レコード会社のファンクラブサイト保守開発（フルリモート）',4,'2021-01-01','2021-01-31','PHP','ファンクラブサイトの、バックエンドのバグ修正。\r\n　数多くの仕様追加、変更の設計～テストの工程を担当していた。','詳細設計、製造、テスト'),(7,5,'保険代理店のLPサイト開発（フルリモート）',2,'2020-12-01','2021-06-30','HTML, CSS, JavaScriot','主に保険商品のLP制作。Psdファイルを元にピクセル単位でLPの作成を担当しました。\r\n　デザイナーとマンツーマン形式で作っていたのでCSSのコーディング作法や実践レベルのスキル向上に繋がった。','製造、テスト'),(8,6,'工業資材販売ECサイトの保守開発',3,'2020-04-01','2020-11-01','PHP','・ECサイトの開発\r\n※新型コロナの影響でブランク期間あり','製造、テスト'),(9,7,'墓所探しサイトの保守運用',4,'2019-08-01','2020-03-01','PHP','Laravelでバックエンド開発を担当。\r\n　他にもインフラ設計（SFTPサーバーの接続環境の構築）なども担当。','製造、テスト'),(10,8,'損保会社の業務システム保守開発',6,'2016-07-01','2019-05-30','Java','新卒入社した年の7月から現場に配属され、製造からテストまで担当。\r\n\r\n・新規元号への移行による保守開発。\r\n・業務システムのサーバー移管業務では夜間に本番環境上でDBの性能比較試験なども担当。','製造、テスト');
/*!40000 ALTER TABLE `work_experience` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-10-02 22:22:28