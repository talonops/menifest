INSERT INTO servers (id, name, token_hash, last_heartbeat, created_at)
VALUES (
    'vps_demo123',
    'Demo Server',
    '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
    strftime('%s', 'now'),
    strftime('%s', 'now')
);