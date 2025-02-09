-- Get All Contact Messages
SELECT * FROM contact_messages;

-- Get Messages from a Specific Email
SELECT * FROM contact_messages WHERE email = 'user1@example.com';

-- Get Messages by Date
SELECT * FROM contact_messages WHERE DATE(created_at) = CURDATE();