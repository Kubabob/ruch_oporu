-- Total Number of Submitted Forms
SELECT COUNT(*) AS total_submissions
FROM form_submissions;

-- Number of Submissions by Status (LGBT, Ally, Innx)
SELECT 
    status, 
    COUNT(*) AS submission_count
FROM form_submissions
GROUP BY status;

-- Number of "Ally" Submissions
SELECT COUNT(*) AS ally_submissions
FROM form_submissions
WHERE status = 'Ally';

-- Number of Submissions with Graphics
SELECT COUNT(*) AS submissions_with_graphics
FROM form_submissions
WHERE is_graphic = TRUE;

-- Number of Submissions with Public Graphics
SELECT COUNT(*) AS submissions_with_public_graphics
FROM form_submissions
WHERE is_public_image = TRUE;

-- Number of Anonymous Submissions
SELECT COUNT(*) AS anonymous_submissions
FROM form_submissions
WHERE is_nonanonymous = FALSE;

-- Number of Submissions with Third-Party Consent
SELECT COUNT(*) AS submissions_with_third_party_consent
FROM form_submissions
WHERE is_another = TRUE;

-- Number of Submissions by Date Range
SELECT 
    DATE(created_at) AS submission_date, 
    COUNT(*) AS submissions_count
FROM form_submissions
WHERE created_at BETWEEN '2023-01-01' AND '2023-12-31'
GROUP BY submission_date
ORDER BY submission_date;

-- Number of Submissions with Authentic Entries
SELECT COUNT(*) AS authentic_submissions
FROM form_submissions
WHERE is_authentic = TRUE;

-- Number of Submissions with Public Consent
SELECT COUNT(*) AS public_consent_submissions
FROM form_submissions
WHERE is_public = TRUE;

-- Number of Submissions with Usage Consent
SELECT COUNT(*) AS usage_consent_submissions
FROM form_submissions
WHERE usage_consent = TRUE;

-- Number of Submissions with RODO Consent
SELECT COUNT(*) AS rodo_consent_submissions
FROM form_submissions
WHERE rodo_consent = TRUE;

-- Number of Submissions with Rules Consent
SELECT COUNT(*) AS rules_consent_submissions
FROM form_submissions
WHERE rules_consent = TRUE;

-- Average Length of User History
SELECT AVG(CHAR_LENGTH(history)) AS avg_history_length
FROM form_submissions;

-- Most Common Titles
SELECT 
    title, 
    COUNT(*) AS title_count
FROM form_submissions
GROUP BY title
ORDER BY title_count DESC
LIMIT 10;

-- Number of Contact Messages
SELECT COUNT(*) AS total_contact_messages
FROM contact_messages;

-- Number of Contact Messages by Email Domain
SELECT 
    SUBSTRING_INDEX(email, '@', -1) AS email_domain, 
    COUNT(*) AS message_count
FROM contact_messages
GROUP BY email_domain
ORDER BY message_count DESC;

-- Number of Contact Messages by Date
SELECT 
    DATE(created_at) AS message_date, 
    COUNT(*) AS message_count
FROM contact_messages
GROUP BY message_date
ORDER BY message_date;

-- Most Frequent Email Senders
SELECT 
    email, 
    COUNT(*) AS message_count
FROM contact_messages
GROUP BY email
ORDER BY message_count DESC
LIMIT 10;

-- Combined Statistics
SELECT 
    (SELECT COUNT(*) FROM form_submissions) AS total_submissions,
    (SELECT COUNT(*) FROM form_submissions WHERE status = 'Ally') AS ally_submissions,
    (SELECT COUNT(*) FROM form_submissions WHERE is_graphic = TRUE) AS submissions_with_graphics,
    (SELECT COUNT(*) FROM contact_messages) AS total_contact_messages;