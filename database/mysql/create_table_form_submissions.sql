-- Table for form submissions (without file content)
CREATE TABLE `form_submissions` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `status` ENUM('LGBT', 'Ally', 'Innx') NOT NULL,
  `history` TEXT NOT NULL,
  `title` VARCHAR(255),
  `quote` TEXT,
  `is_graphic` BOOLEAN NOT NULL DEFAULT FALSE,
  `graphic_file` VARCHAR(255), -- Stores filename/path for graphic
  `is_another` BOOLEAN NOT NULL DEFAULT FALSE,
  `image_consent_file` VARCHAR(255), -- Stores filename/path for consent document
  `is_public_image` BOOLEAN NOT NULL DEFAULT FALSE,
  `is_nonanonymous` BOOLEAN NOT NULL DEFAULT FALSE,
  `signature` VARCHAR(255),
  `is_authentic` BOOLEAN NOT NULL DEFAULT FALSE,
  `is_public` BOOLEAN NOT NULL DEFAULT FALSE,
  `usage_consent` BOOLEAN NOT NULL DEFAULT FALSE,
  `rules_consent` BOOLEAN NOT NULL DEFAULT FALSE,
  `rodo_consent` BOOLEAN NOT NULL DEFAULT FALSE,
  `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
  INDEX `status_index` (`status`),
  INDEX `created_at_index` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;