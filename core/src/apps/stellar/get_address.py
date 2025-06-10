from typing import TYPE_CHECKING

from apps.common.keychain import auto_keychain

if TYPE_CHECKING:
    from trezor.messages import StellarAddress, StellarGetAddress
    from trezor.wire import MaybeEarlyResponse

    from apps.common.keychain import Keychain


@auto_keychain(__name__)
async def get_address(
    msg: StellarGetAddress, keychain: Keychain
) -> MaybeEarlyResponse[StellarAddress]:
    from trezor.messages import StellarAddress
    from trezor.ui.layouts import show_address, show_continue_in_app
    from trezor.wire import early_response

    from apps.common import paths, seed

    from . import helpers

    address_n = msg.address_n  # local_cache_attribute

    await paths.validate_path(keychain, address_n)

    node = keychain.derive(address_n)
    pubkey = seed.remove_ed25519_prefix(node.public_key())
    address = helpers.address_from_public_key(pubkey)
    response = StellarAddress(address=address)

    if msg.show_display:
        from trezor import TR

        from . import PATTERN, SLIP44_ID

        await show_address(
            address,
            subtitle=f"XLM {TR.words__address}",
            case_sensitive=False,
            path=paths.address_n_to_str(address_n),
            account=paths.get_account_name("XLM", msg.address_n, PATTERN, SLIP44_ID),
            chunkify=bool(msg.chunkify),
        )
        return await early_response(
            response, show_continue_in_app(TR.address__confirmed)
        )

    return response
