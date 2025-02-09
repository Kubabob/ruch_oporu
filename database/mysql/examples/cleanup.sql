-- Delete All Test Data
DELETE FROM form_submissions;
DELETE FROM contact_messages;

-- Reset Auto Increment
ALTER TABLE form_submissions AUTO_INCREMENT = 1;
ALTER TABLE contact_messages AUTO_INCREMENT = 1;