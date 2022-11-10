def test_utf8_data(self):
    self.write_line(
        'MAIL From: naïve@examplé BODY=8BITMIME SMTPUTF8'.encode('utf-8'))
    self.assertEqual(self.channel.socket.last[0:3], b'250')