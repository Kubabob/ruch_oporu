-- Insert a submission with a graphic and third-party consent
INSERT INTO form_submissions (
    status, history, title, quote, is_graphic, graphic_file, 
    is_another, image_consent_file, is_public_image, is_nonanonymous, 
    signature, is_authentic, is_public, usage_consent, rules_consent, rodo_consent
) VALUES (
    'LGBT', 'This is my personal story about...', 'My Journey', 
    'Stay true to yourself', TRUE, 'a1b2c3d4-image.jpg', 
    TRUE, 'e5f6g7h8-consent.pdf', TRUE, FALSE, 
    NULL, TRUE, TRUE, TRUE, TRUE, TRUE
);

-- Insert a submission without a graphic
INSERT INTO form_submissions (
    status, history, title, quote, is_graphic, graphic_file, 
    is_another, image_consent_file, is_public_image, is_nonanonymous, 
    signature, is_authentic, is_public, usage_consent, rules_consent, rodo_consent
) VALUES (
    'Ally', 'As an ally, I support...', 'Supporting the Community', 
    'Love is love', FALSE, NULL, 
    FALSE, NULL, FALSE, TRUE, 
    'John Doe', TRUE, TRUE, TRUE, TRUE, TRUE
);

-- Insert an anonymous submission
INSERT INTO form_submissions (
    status, history, title, quote, is_graphic, graphic_file, 
    is_another, image_consent_file, is_public_image, is_nonanonymous, 
    signature, is_authentic, is_public, usage_consent, rules_consent, rodo_consent
) VALUES (
    'Innx', 'My experience as a non-binary person...', 'Finding Myself', 
    'Be yourself', FALSE, NULL, 
    FALSE, NULL, FALSE, FALSE, 
    NULL, TRUE, TRUE, TRUE, TRUE, TRUE
);