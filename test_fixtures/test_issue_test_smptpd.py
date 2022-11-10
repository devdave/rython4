def test_utf8_data(self):
    self.write_line(b'EHLO example')
    self.write_line(
        'MAIL From: naïve@examplé BODY=8BITMIME SMTPUTF8'.encode('utf-8'))
    self.assertEqual(self.channel.socket.last[0:3], b'250')
    self.write_line('RCPT To:späm@examplé'.encode('utf-8'))
    self.assertEqual(self.channel.socket.last[0:3], b'250')
    self.write_line(b'DATA')
    self.assertEqual(self.channel.socket.last[0:3], b'354')
    self.write_line(b'utf8 enriched text: \xc5\xbc\xc5\xba\xc4\x87')
    self.write_line(b'.')
    self.assertEqual(
        self.channel.received_data,
        b'utf8 enriched text: \xc5\xbc\xc5\xba\xc4\x87')