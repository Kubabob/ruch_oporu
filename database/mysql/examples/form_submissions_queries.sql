-- Get All Submissions
SELECT * FROM form_submissions;

-- Get Submissions with Graphics
SELECT * FROM form_submissions WHERE is_graphic = TRUE;

-- Get Submissions by Status
SELECT * FROM form_submissions WHERE status = 'LGBT';

-- Get Submissions with Public Graphics
SELECT * FROM form_submissions WHERE is_public_image = TRUE;

-- Get Submissions with Public Graphics
SELECT * FROM form_submissions WHERE is_public_image = TRUE;

-- Get Anonymous Submissions
SELECT * FROM form_submissions WHERE is_nonanonymous = FALSE;