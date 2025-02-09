-- Total Number of Submissions and Contact Messages
SELECT 
    (SELECT COUNT(*) FROM form_submissions) AS total_submissions,
    (SELECT COUNT(*) FROM contact_messages) AS total_contact_messages;

-- Submissions with Graphics and Contact Messages from a Specific Domain
SELECT 
    (SELECT COUNT(*) FROM form_submissions WHERE is_graphic = TRUE) AS submissions_with_graphics,
    (SELECT COUNT(*) FROM contact_messages WHERE email LIKE '%@example.com') AS messages_from_example_domain;
