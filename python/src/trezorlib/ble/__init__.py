import typing as t

from .. import messages

if t.TYPE_CHECKING:
    from ..transport.session import Session


def update(
    session: "Session",
    datfile: bytes,
    binfile: bytes,
    progress_update: t.Callable[[int], t.Any] = lambda _: None,
):
    raise NotImplementedError()
    # chunk_len = 4096
    # offset = 0

    # resp = session.call(
    #     messages.UploadBLEFirmwareInit(init_data=datfile, binsize=len(binfile))
    # )

    # while isinstance(resp, messages.UploadBLEFirmwareNextChunk):

    #     payload = binfile[offset : offset + chunk_len]
    #     resp = session.call(messages.UploadBLEFirmwareChunk(data=payload))
    #     progress_update(chunk_len)
    #     offset += chunk_len

    # if isinstance(resp, messages.Success):
    #     return
    # else:
    #     raise RuntimeError(f"Unexpected message {resp}")


def erase_bonds(
    session: "Session",
):

    resp = session.call(messages.EraseBonds())

    if isinstance(resp, messages.Success):
        return
    else:
        raise RuntimeError(f"Unexpected message {resp}")


def unpair(
    session: "Session",
):

    resp = session.call(messages.Unpair())

    if isinstance(resp, messages.Success):
        return
    else:
        raise RuntimeError(f"Unexpected message {resp}")


def disconnect(
    session: "Session",
):
    resp = session.call(messages.Disconnect())

    if isinstance(resp, messages.Success):
        return
    else:
        raise RuntimeError(f"Unexpected message {resp}")
