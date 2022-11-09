async def _sock_sendfile_fallback(self, sock, file, offset, count):
    if offset:
        file.seek(offset)
    blocksize = (
        min(count, constants.SENDFILE_FALLBACK_READBUFFER_SIZE)
        if count else constants.SENDFILE_FALLBACK_READBUFFER_SIZE
    )
    buf = bytearray(blocksize)
    total_sent = 0
    try:
        while True:
            if count:
                blocksize = min(count - total_sent, blocksize)
                if blocksize <= 0:
                    break
            view = memoryview(buf)[:blocksize]
            read = await self.run_in_executor(None, file.readinto, view)
            if not read:
                break  # EOF
            await self.sock_sendall(sock, view[:read])
            total_sent += read
        return total_sent
    finally:
        if total_sent > 0 and hasattr(file, 'seek'):
            file.seek(offset + total_sent)