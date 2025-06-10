from typing import TYPE_CHECKING

from apps.common.keychain import with_slip44_keychain

from . import CURVE, PATTERNS, SLIP44_ID

if TYPE_CHECKING:
    from trezor.messages import TezosGetPublicKey, TezosPublicKey
    from trezor.wire import MaybeEarlyResponse

    from apps.common.keychain import Keychain


@with_slip44_keychain(*PATTERNS, slip44_id=SLIP44_ID, curve=CURVE)
async def get_public_key(
    msg: TezosGetPublicKey, keychain: Keychain
) -> MaybeEarlyResponse[TezosPublicKey]:
    from trezor.messages import TezosPublicKey
    from trezor.ui.layouts import show_continue_in_app, show_pubkey
    from trezor.wire import early_response

    from apps.common import paths, seed

    from . import helpers

    await paths.validate_path(keychain, msg.address_n)

    node = keychain.derive(msg.address_n)
    pk = seed.remove_ed25519_prefix(node.public_key())
    pk_prefixed = helpers.base58_encode_check(pk, helpers.TEZOS_PUBLICKEY_PREFIX)
    response = TezosPublicKey(public_key=pk_prefixed)

    if msg.show_display:
        from trezor import TR

        from . import PATTERNS, SLIP44_ID

        account = paths.get_account_name("XTZ", msg.address_n, PATTERNS, SLIP44_ID)
        path = paths.address_n_to_str(msg.address_n)
        await show_pubkey(pk_prefixed, subtitle=account, account=account, path=path)
        return await early_response(
            response, show_continue_in_app(TR.address__public_key_confirmed)
        )

    return response
