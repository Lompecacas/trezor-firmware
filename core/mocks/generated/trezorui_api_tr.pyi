from typing import *

    def multiple_pages_texts(
        *,
        title: str,
        verb: str,
        items: list[str],
    ) -> LayoutObj[UiResult]:
        """Show multiple texts, each on its own page. TR specific."""
