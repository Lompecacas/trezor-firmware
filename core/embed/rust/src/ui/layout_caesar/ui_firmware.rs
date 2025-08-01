use core::cmp::Ordering;

use crate::{
    error::Error,
    io::BinaryData,
    maybe_trace::MaybeTrace,
    micropython::{buffer::StrBuffer, gc::Gc, iter::IterBuf, list::List, obj::Obj, util},
    strutil::TString,
    translations::TR,
    ui::{
        component::{
            connect::Connect,
            text::{
                op::OpTextLayout,
                paragraphs::{
                    Checklist, Paragraph, ParagraphSource, ParagraphVecLong, ParagraphVecShort,
                    Paragraphs, VecExt,
                },
                TextStyle,
            },
            Component, ComponentExt, Empty, FormattedText, Label, LineBreaking, Paginate, Timeout,
        },
        geometry,
        layout::{
            obj::{LayoutMaybeTrace, LayoutObj, RootComponent},
            util::{ConfirmValueParams, RecoveryType},
        },
        ui_firmware::{
            FirmwareUI, ERROR_NOT_IMPLEMENTED, MAX_CHECKLIST_ITEMS, MAX_GROUP_SHARE_LINES,
            MAX_MENU_ITEMS, MAX_WORD_QUIZ_ITEMS,
        },
        ModelUI,
    },
};

use super::{
    component::{
        AddressDetails, ButtonActions, ButtonDetails, ButtonLayout, ButtonPage, ChoiceControls,
        CoinJoinProgress, ConfirmHomescreen, Flow, FlowPages, Frame, Homescreen, Lockscreen,
        NumberInput, Page, PassphraseEntry, PinEntry, Progress, ScrollableFrame, ShareWords,
        ShowMore, SimpleChoice, WordlistEntry, WordlistType,
    },
    constant, fonts, theme, UICaesar,
};

use heapless::Vec;

impl FirmwareUI for UICaesar {
    fn confirm_action(
        title: TString<'static>,
        action: Option<TString<'static>>,
        description: Option<TString<'static>>,
        _subtitle: Option<TString<'static>>,
        verb: Option<TString<'static>>,
        verb_cancel: Option<TString<'static>>,
        hold: bool,
        _hold_danger: bool,
        reverse: bool,
        _prompt_screen: bool,
        _prompt_title: Option<TString<'static>>,
        _external_menu: bool, // TODO: will eventually replace the internal menu
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let paragraphs = {
            let action = action.unwrap_or("".into());
            let description = description.unwrap_or("".into());
            let mut paragraphs = ParagraphVecShort::new();
            if !reverse {
                paragraphs
                    .add(Paragraph::new(&theme::TEXT_BOLD, action))
                    .add(Paragraph::new(&theme::TEXT_NORMAL, description));
            } else {
                paragraphs
                    .add(Paragraph::new(&theme::TEXT_NORMAL, description))
                    .add(Paragraph::new(&theme::TEXT_BOLD, action));
            }
            paragraphs.into_paragraphs()
        };

        content_in_button_page(
            title,
            paragraphs,
            verb.unwrap_or(TString::empty()),
            verb_cancel,
            hold,
        )
    }

    fn confirm_address(
        title: TString<'static>,
        address: Obj,
        address_label: Option<TString<'static>>,
        verb: Option<TString<'static>>,
        info_button: bool,
        chunkify: bool,
    ) -> Result<Gc<LayoutObj>, Error> {
        let verb = verb.unwrap_or(TR::buttons__confirm.into());
        let address: TString = address.try_into()?;

        let get_page = move |page_index| {
            assert!(page_index == 0);
            let (btn_layout, btn_actions) = if info_button {
                (
                    ButtonLayout::cancel_armed_info(verb),
                    ButtonActions::cancel_confirm_info(),
                )
            } else {
                (
                    ButtonLayout::cancel_none_text(verb),
                    ButtonActions::cancel_none_confirm(),
                )
            };
            let mut ops = OpTextLayout::new(theme::TEXT_MONO_DATA);
            if let Some(label) = address_label {
                // NOTE: need to explicitly turn off the chunkification before rendering the
                // address label (for some reason it does not help to turn it off after
                // rendering the chunks)
                if chunkify {
                    ops.add_chunkify_text(None);
                }
                ops.add_text(label, fonts::FONT_NORMAL).add_newline();
            }
            if chunkify {
                // Chunkifying the address into smaller pieces when requested
                ops.add_chunkify_text(Some((theme::MONO_CHUNKS, 2)));
            }
            ops.add_text(address, fonts::FONT_MONO);
            let formatted = FormattedText::new(ops).vertically_centered();
            Page::new(btn_layout, btn_actions, formatted).with_title(title)
        };
        let pages = FlowPages::new(get_page, 1);

        let obj = LayoutObj::new(Flow::new(pages))?;
        Ok(obj)
    }

    fn confirm_value(
        title: TString<'static>,
        value: Obj,
        description: Option<TString<'static>>,
        is_data: bool,
        extra: Option<TString<'static>>,
        _subtitle: Option<TString<'static>>,
        verb: Option<TString<'static>>,
        verb_cancel: Option<TString<'static>>,
        _info: bool,
        hold: bool,
        chunkify: bool,
        _page_counter: bool,
        _prompt_screen: bool,
        _cancel: bool,
        _warning_footer: Option<TString<'static>>,
    ) -> Result<Gc<LayoutObj>, Error> {
        let paragraphs = ConfirmValueParams {
            description: description.unwrap_or("".into()),
            extra: extra.unwrap_or("".into()),
            value: value.try_into()?,
            font: if chunkify {
                // Chunkifying the address into smaller pieces when requested
                &theme::TEXT_MONO_ADDRESS_CHUNKS
            } else if is_data {
                &theme::TEXT_MONO_DATA
            } else {
                &theme::TEXT_NORMAL
            },
            description_font: &theme::TEXT_BOLD,
            extra_font: &theme::TEXT_NORMAL,
        }
        .into_paragraphs();

        let layout = content_in_button_page(
            title,
            paragraphs,
            verb.unwrap_or(TR::buttons__confirm.into()),
            verb_cancel,
            hold,
        )?;
        LayoutObj::new_root(layout)
    }

    fn confirm_value_intro(
        _title: TString<'static>,
        _value: Obj,
        _subtitle: Option<TString<'static>>,
        _verb: Option<TString<'static>>,
        _verb_cancel: Option<TString<'static>>,
        _hold: bool,
        _chunkify: bool,
    ) -> Result<Gc<LayoutObj>, Error> {
        Err::<Gc<LayoutObj>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn confirm_homescreen(
        title: TString<'static>,
        image: BinaryData<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(ConfirmHomescreen::new(title, image));
        Ok(layout)
    }

    fn confirm_coinjoin(
        max_rounds: TString<'static>,
        max_feerate: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        // Decreasing bottom padding between paragraphs to fit one screen
        let paragraphs = Paragraphs::new([
            Paragraph::new(&theme::TEXT_BOLD, TR::coinjoin__max_rounds).with_bottom_padding(2),
            Paragraph::new(&theme::TEXT_MONO, max_rounds),
            Paragraph::new(&theme::TEXT_BOLD, TR::coinjoin__max_mining_fee)
                .with_bottom_padding(2)
                .no_break(),
            Paragraph::new(&theme::TEXT_MONO, max_feerate).with_bottom_padding(2),
        ]);

        content_in_button_page(
            TR::coinjoin__title.into(),
            paragraphs,
            TR::buttons__hold_to_confirm.into(),
            None,
            true,
        )
    }

    fn confirm_emphasized(
        _title: TString<'static>,
        _items: Obj,
        _verb: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn confirm_fido(
        title: TString<'static>,
        app_name: TString<'static>,
        _icon: Option<TString<'static>>,
        accounts: Gc<List>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        // Cache the page count so that we can move `accounts` into the closure.
        let page_count = accounts.len();

        // Closure to lazy-load the information on given page index.
        // Done like this to allow arbitrarily many pages without
        // the need of any allocation here in Rust.
        let get_page = move |page_index| {
            let account_obj = unwrap!(accounts.get(page_index));
            let account = TString::try_from(account_obj).unwrap_or_else(|_| TString::empty());

            let (btn_layout, btn_actions) = if page_count == 1 {
                // There is only one page
                (
                    ButtonLayout::cancel_none_text(TR::buttons__confirm.into()),
                    ButtonActions::cancel_none_confirm(),
                )
            } else if page_index == 0 {
                // First page
                (
                    ButtonLayout::cancel_armed_arrow(TR::buttons__select.into()),
                    ButtonActions::cancel_confirm_next(),
                )
            } else if page_index == page_count - 1 {
                // Last page
                (
                    ButtonLayout::arrow_armed_none(TR::buttons__select.into()),
                    ButtonActions::prev_confirm_none(),
                )
            } else {
                // Page in the middle
                (
                    ButtonLayout::arrow_armed_arrow(TR::buttons__select.into()),
                    ButtonActions::prev_confirm_next(),
                )
            };

            let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
            ops.add_newline()
                .add_text(app_name, fonts::FONT_NORMAL)
                .add_newline()
                .add_text(account, fonts::FONT_BOLD);
            let formatted = FormattedText::new(ops);

            Page::new(btn_layout, btn_actions, formatted)
        };

        let pages = FlowPages::new(get_page, page_count);
        // Returning the page index in case of confirmation.
        let obj = RootComponent::new(
            Flow::new(pages)
                .with_common_title(title)
                .with_return_confirmed_index(),
        );
        Ok(obj)
    }

    fn confirm_firmware_update(
        description: TString<'static>,
        fingerprint: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        use super::component::bl_confirm::Confirm;
        let title = TR::firmware_update__title;
        let message = Label::left_aligned(description, theme::TEXT_NORMAL).vertically_centered();
        let fingerprint = Label::left_aligned(
            fingerprint,
            theme::TEXT_NORMAL.with_line_breaking(LineBreaking::BreakWordsNoHyphen),
        )
        .vertically_centered();

        let layout = RootComponent::new(
            Confirm::new(
                theme::BG,
                title.into(),
                message,
                None,
                TR::buttons__install.as_tstring(),
                false,
            )
            .with_info_screen(
                TR::firmware_update__title_fingerprint.as_tstring(),
                fingerprint,
            ),
        );
        Ok(layout)
    }

    fn confirm_modify_fee(
        _title: TString<'static>,
        sign: i32,
        user_fee_change: TString<'static>,
        total_fee_new: TString<'static>,
        fee_rate_amount: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let (description, change) = match sign {
            s if s < 0 => (TR::modify_fee__decrease_fee, user_fee_change),
            s if s > 0 => (TR::modify_fee__increase_fee, user_fee_change),
            _ => (TR::modify_fee__no_change, "".into()),
        };

        let mut paragraphs_vec = ParagraphVecShort::new();
        paragraphs_vec
            .add(Paragraph::new(&theme::TEXT_BOLD, description))
            .add(Paragraph::new(&theme::TEXT_MONO, change))
            .add(Paragraph::new(&theme::TEXT_BOLD, TR::modify_fee__transaction_fee).no_break())
            .add(Paragraph::new(&theme::TEXT_MONO, total_fee_new));

        if let Some(fee_rate_amount) = fee_rate_amount {
            paragraphs_vec
                .add(Paragraph::new(&theme::TEXT_BOLD, TR::modify_fee__fee_rate).no_break())
                .add(Paragraph::new(&theme::TEXT_MONO, fee_rate_amount));
        }

        content_in_button_page(
            TR::modify_fee__title.into(),
            paragraphs_vec.into_paragraphs(),
            TR::buttons__confirm.into(),
            Some("".into()),
            false,
        )
    }

    fn confirm_modify_output(
        sign: i32,
        amount_change: TString<'static>,
        amount_new: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let description = if sign < 0 {
            TR::modify_amount__decrease_amount
        } else {
            TR::modify_amount__increase_amount
        };

        let paragraphs = Paragraphs::new([
            Paragraph::new(&theme::TEXT_NORMAL, description),
            Paragraph::new(&theme::TEXT_MONO, amount_change).break_after(),
            Paragraph::new(&theme::TEXT_BOLD, TR::modify_amount__new_amount),
            Paragraph::new(&theme::TEXT_MONO, amount_new),
        ]);

        content_in_button_page(
            TR::modify_amount__title.into(),
            paragraphs,
            TR::buttons__confirm.into(),
            Some("".into()),
            false,
        )
    }

    fn confirm_more(
        title: TString<'static>,
        button: TString<'static>,
        _button_style_confirm: bool,
        _hold: bool,
        items: Obj,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut paragraphs = ParagraphVecLong::new();

        for para in IterBuf::new().try_iterate(items)? {
            let [text, is_data]: [Obj; 2] = util::iter_into_array(para)?;
            let is_data = is_data.try_into()?;
            let style: &TextStyle = if is_data {
                &theme::TEXT_MONO_DATA
            } else {
                &theme::TEXT_NORMAL
            };
            let text: TString = text.try_into()?;
            paragraphs.add(Paragraph::new(style, text));
        }

        content_in_button_page(
            title,
            paragraphs.into_paragraphs(),
            button,
            Some("<".into()),
            false,
        )
    }

    fn confirm_properties(
        title: TString<'static>,
        _subtitle: Option<TString<'static>>,
        items: Obj,
        hold: bool,
        _verb: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let paragraphs = parse_properties(items)?;

        let button_text = if hold {
            TR::buttons__hold_to_confirm.into()
        } else {
            TR::buttons__confirm.into()
        };

        content_in_button_page(
            title,
            paragraphs.into_paragraphs(),
            button_text,
            Some("".into()),
            hold,
        )
    }

    fn confirm_reset_device(recovery: bool) -> Result<impl LayoutMaybeTrace, Error> {
        let (title, button) = if recovery {
            (
                TR::recovery__title_recover.into(),
                TR::reset__button_recover.into(),
            )
        } else {
            (
                TR::reset__title_create_wallet.into(),
                TR::reset__button_create.into(),
            )
        };
        let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
        ops.add_text(TR::reset__by_continuing, fonts::FONT_NORMAL)
            .add_next_page()
            .add_text(TR::reset__more_info_at, fonts::FONT_NORMAL)
            .add_newline()
            .add_text(TR::reset__tos_link, fonts::FONT_BOLD);
        let formatted = FormattedText::new(ops).vertically_centered();

        content_in_button_page(title, formatted, button, Some("".into()), false)
    }

    fn confirm_summary(
        amount: Option<TString<'static>>,
        amount_label: Option<TString<'static>>,
        fee: TString<'static>,
        fee_label: TString<'static>,
        title: Option<TString<'static>>,
        account_items: Option<Obj>,
        account_title: Option<TString<'static>>,
        extra_items: Option<Obj>,
        extra_title: Option<TString<'static>>,
        verb_cancel: Option<TString<'static>>,
        external_menu: bool, // TODO: will eventually replace the internal menu
    ) -> Result<impl LayoutMaybeTrace, Error> {
        // collect available info pages
        let mut info_pages: Vec<(TString, Obj), 2> = Vec::new();
        if let Some(info) = extra_items {
            // put extra items first as it's typically used for fee info
            let extra_title = extra_title.unwrap_or(TR::words__title_information.into());
            unwrap!(info_pages.push((extra_title, info)));
        }
        if let Some(info) = account_items {
            let account_title =
                account_title.unwrap_or(TR::confirm_total__title_sending_from.into());
            unwrap!(info_pages.push((account_title, info)));
        }
        if external_menu && !info_pages.is_empty() {
            return Err(Error::ValueError(
                c"Cannot use both info pages and external menu",
            ));
        }

        // button layouts and actions
        let verb_cancel: TString = verb_cancel.unwrap_or(TString::empty());
        let btns_summary_page = move |has_pages_after: bool| -> (ButtonLayout, ButtonActions) {
            // if there are no info pages, the right button is not needed
            // if verb_cancel is "^", the left button is an arrow pointing up
            let left_btn = Some(ButtonDetails::from_text_possible_icon(verb_cancel));
            let right_btn = (has_pages_after || external_menu).then(|| {
                ButtonDetails::text("i".into())
                    .with_fixed_width(theme::BUTTON_ICON_WIDTH)
                    .with_font(fonts::FONT_NORMAL)
            });
            let middle_btn = Some(ButtonDetails::armed_text(TR::buttons__confirm.into()));

            (
                ButtonLayout::new(left_btn, middle_btn, right_btn),
                if has_pages_after {
                    ButtonActions::cancel_confirm_next()
                } else if external_menu {
                    ButtonActions::cancel_confirm_info()
                } else {
                    ButtonActions::cancel_confirm_none()
                },
            )
        };
        let btns_info_page = |is_last: bool| -> (ButtonLayout, ButtonActions) {
            // on the last info page, the right button is not needed
            if is_last {
                (
                    ButtonLayout::arrow_none_none(),
                    ButtonActions::prev_none_none(),
                )
            } else {
                (
                    ButtonLayout::arrow_none_arrow(),
                    ButtonActions::prev_none_next(),
                )
            }
        };

        let total_pages = 1 + info_pages.len();
        let get_page = move |page_index| {
            match page_index {
                0 => {
                    // Total amount + fee
                    let (btn_layout, btn_actions) = btns_summary_page(!info_pages.is_empty());

                    let mut ops = OpTextLayout::new(theme::TEXT_MONO);
                    if let Some(title) = title {
                        ops.add_text(title, fonts::FONT_BOLD_UPPER).add_newline();
                    }

                    let mut has_amount = false;
                    if let Some(amount) = amount {
                        if let Some(amount_label) = amount_label {
                            has_amount = true;
                            ops.add_text(amount_label, fonts::FONT_BOLD)
                                .add_newline()
                                .add_text(amount, fonts::FONT_MONO);
                        }
                    }

                    if !fee_label.is_empty() || !fee.is_empty() {
                        if has_amount {
                            ops.add_newline();
                        }
                        ops.add_newline()
                            .add_text(fee_label, fonts::FONT_BOLD)
                            .add_newline()
                            .add_text(fee, fonts::FONT_MONO);
                    }

                    let formatted = FormattedText::new(ops);
                    Page::new(btn_layout, btn_actions, formatted)
                }
                i => {
                    // Other info pages as provided
                    let (title, info_obj) = &info_pages[i - 1];
                    let is_last = i == total_pages - 1;
                    let (btn_layout, btn_actions) = btns_info_page(is_last);

                    let mut ops = OpTextLayout::new(theme::TEXT_MONO);
                    for item in unwrap!(IterBuf::new().try_iterate(*info_obj)) {
                        let [key, value]: [Obj; 2] = unwrap!(util::iter_into_array(item));
                        if !ops.is_empty() {
                            // Each key-value pair is on its own page
                            ops.add_next_page();
                        }
                        ops.add_text(unwrap!(TString::try_from(key)), fonts::FONT_BOLD)
                            .add_newline()
                            .add_text(unwrap!(TString::try_from(value)), fonts::FONT_MONO);
                    }

                    let formatted = FormattedText::new(ops).vertically_centered();
                    Page::new(btn_layout, btn_actions, formatted)
                        .with_slim_arrows()
                        .with_title(*title)
                }
            }
        };
        let pages = FlowPages::new(get_page, total_pages);

        let layout = RootComponent::new(
            Flow::new(pages)
                .with_scrollbar(false)
                .with_menu(external_menu),
        );
        Ok(layout)
    }

    fn confirm_with_info(
        title: TString<'static>,
        _subtitle: Option<TString<'static>>,
        items: Obj,
        verb: TString<'static>,
        _verb_info: TString<'static>,
        verb_cancel: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut paragraphs = ParagraphVecShort::new();

        for para in IterBuf::new().try_iterate(items)? {
            let [text, is_data]: [Obj; 2] = util::iter_into_array(para)?;
            let is_data = is_data.try_into()?;
            let style: &TextStyle = if is_data {
                &theme::TEXT_MONO_DATA_WITH_CLASSIC_ELLIPSIS
            } else {
                &theme::TEXT_NORMAL
            };
            let text: TString = text.try_into()?;
            paragraphs.add(Paragraph::new(style, text));
            if paragraphs.is_full() {
                break;
            }
        }

        let layout = RootComponent::new(Frame::new(
            title,
            ShowMore::<Paragraphs<ParagraphVecShort>>::new(
                paragraphs.into_paragraphs(),
                verb_cancel,
                verb,
            ),
        ));
        Ok(layout)
    }

    fn check_homescreen_format(image: BinaryData, _accept_toif: bool) -> bool {
        super::component::check_homescreen_format(image)
    }

    fn continue_recovery_homepage(
        text: TString<'static>,
        _subtext: Option<TString<'static>>,
        button: Option<TString<'static>>,
        recovery_type: RecoveryType,
        show_instructions: bool,
        _remaining_shares: Option<crate::micropython::obj::Obj>,
    ) -> Result<Gc<LayoutObj>, Error> {
        let mut paragraphs = ParagraphVecShort::new();
        let button = button.unwrap_or(TString::empty());
        paragraphs.add(Paragraph::new(&theme::TEXT_NORMAL, text));
        if show_instructions {
            paragraphs
                .add(Paragraph::new(
                    &theme::TEXT_NORMAL,
                    TR::recovery__enter_each_word,
                ))
                .add(Paragraph::new(
                    &theme::TEXT_NORMAL,
                    TR::recovery__cursor_will_change,
                ));
        }

        let title = match recovery_type {
            RecoveryType::DryRun => TR::recovery__title_dry_run,
            RecoveryType::UnlockRepeatedBackup => TR::recovery__title_dry_run,
            _ => TR::recovery__title,
        };

        let layout = content_in_button_page(
            title.into(),
            paragraphs.into_paragraphs(),
            button,
            Some("".into()),
            false,
        )?;
        LayoutObj::new_root(layout)
    }

    fn flow_confirm_output(
        _title: Option<TString<'static>>,
        _subtitle: Option<TString<'static>>,
        _description: Option<TString<'static>>,
        _extra: Option<TString<'static>>,
        _message: Obj,
        _amount: Option<Obj>,
        _chunkify: bool,
        _text_mono: bool,
        _account_title: TString<'static>,
        _account: Option<TString<'static>>,
        _account_path: Option<TString<'static>>,
        _br_code: u16,
        _br_name: TString<'static>,
        _address_item: Option<(TString<'static>, Obj)>,
        _extra_item: Option<(TString<'static>, Obj)>,
        _summary_items: Option<Obj>,
        _fee_items: Option<Obj>,
        _summary_title: Option<TString<'static>>,
        _summary_br_code: Option<u16>,
        _summary_br_name: Option<TString<'static>>,
        _cancel_text: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn flow_confirm_set_new_pin(
        _title: TString<'static>,
        _description: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn flow_get_address(
        _address: TString<'static>,
        _title: TString<'static>,
        _subtitle: Option<TString<'static>>,
        _description: Option<TString<'static>>,
        _hint: Option<TString<'static>>,
        _chunkify: bool,
        _address_qr: TString<'static>,
        _case_sensitive: bool,
        _account: Option<TString<'static>>,
        _path: Option<TString<'static>>,
        _xpubs: Obj,
        _br_code: u16,
        _br_name: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn flow_get_pubkey(
        _pubkey: TString<'static>,
        _title: TString<'static>,
        _subtitle: Option<TString<'static>>,
        _hint: Option<TString<'static>>,
        _pubkey_qr: TString<'static>,
        _account: Option<TString<'static>>,
        _path: Option<TString<'static>>,
        _br_code: u16,
        _br_name: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn multiple_pages_texts(
        title: TString<'static>,
        verb: TString<'static>,
        items: Gc<List>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        // Cache the page count so that we can move `items` into the closure.
        let page_count = items.len();

        // Closure to lazy-load the information on given page index.
        // Done like this to allow arbitrarily many pages without
        // the need of any allocation here in Rust.
        let get_page = move |page_index| {
            let item_obj = unwrap!(items.get(page_index));
            let text = unwrap!(TString::try_from(item_obj));

            let (btn_layout, btn_actions) = if page_count == 1 {
                // There is only one page
                (
                    ButtonLayout::cancel_none_text(verb),
                    ButtonActions::cancel_none_confirm(),
                )
            } else if page_index == 0 {
                // First page
                (
                    ButtonLayout::cancel_none_arrow_wide(),
                    ButtonActions::cancel_none_next(),
                )
            } else if page_index == page_count - 1 {
                // Last page
                (
                    ButtonLayout::up_arrow_none_text(verb),
                    ButtonActions::prev_none_confirm(),
                )
            } else {
                // Page in the middle
                (
                    ButtonLayout::up_arrow_none_arrow_wide(),
                    ButtonActions::prev_none_next(),
                )
            };

            let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
            ops.add_text(text, fonts::FONT_NORMAL);
            let formatted = FormattedText::new(ops).vertically_centered();

            Page::new(btn_layout, btn_actions, formatted)
        };

        let pages = FlowPages::new(get_page, page_count);
        let layout = RootComponent::new(Flow::new(pages).with_common_title(title));
        Ok(layout)
    }

    fn prompt_backup() -> Result<impl LayoutMaybeTrace, Error> {
        let get_page = move |page_index| match page_index {
            0 => {
                let btn_layout = ButtonLayout::text_none_arrow_wide(TR::buttons__skip.into());
                let btn_actions = ButtonActions::cancel_none_next();
                let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
                ops.add_text(TR::backup__new_wallet_created, fonts::FONT_NORMAL)
                    .add_newline()
                    .add_text(TR::backup__it_should_be_backed_up_now, fonts::FONT_NORMAL);
                let formatted = FormattedText::new(ops).vertically_centered();
                Page::new(btn_layout, btn_actions, formatted)
                    .with_title(TR::words__title_success.into())
            }
            1 => {
                let btn_layout = ButtonLayout::up_arrow_none_text(TR::buttons__back_up.into());
                let btn_actions = ButtonActions::prev_none_confirm();
                let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
                ops.add_text(TR::backup__recover_anytime, fonts::FONT_NORMAL);
                let formatted = FormattedText::new(ops).vertically_centered();
                Page::new(btn_layout, btn_actions, formatted)
                    .with_title(TR::backup__title_backup_wallet.into())
            }
            _ => unreachable!(),
        };
        let pages = FlowPages::new(get_page, 2);

        let layout = RootComponent::new(Flow::new(pages));
        Ok(layout)
    }

    fn request_bip39(
        prompt: TString<'static>,
        prefill_word: TString<'static>,
        can_go_back: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(
            Frame::new(
                prompt,
                prefill_word
                    .map(|s| WordlistEntry::prefilled_word(s, WordlistType::Bip39, can_go_back)),
            )
            .with_title_centered(),
        );
        Ok(layout)
    }

    fn request_slip39(
        prompt: TString<'static>,
        prefill_word: TString<'static>,
        can_go_back: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(
            Frame::new(
                prompt,
                prefill_word
                    .map(|s| WordlistEntry::prefilled_word(s, WordlistType::Slip39, can_go_back)),
            )
            .with_title_centered(),
        );
        Ok(layout)
    }

    fn request_number(
        title: TString<'static>,
        count: u32,
        min_count: u32,
        max_count: u32,
        _description: Option<TString<'static>>,
        _more_info_callback: Option<impl Fn(u32) -> TString<'static> + 'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(
            Frame::new(title, NumberInput::new(min_count, max_count, count)).with_title_centered(),
        );
        Ok(layout)
    }

    fn request_duration(
        _title: TString<'static>,
        _duration_ms: u32,
        _min_ms: u32,
        _max_ms: u32,
        _description: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn request_pin(
        prompt: TString<'static>,
        subprompt: TString<'static>,
        _allow_cancel: bool,
        _warning: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(PinEntry::new(prompt, subprompt));
        Ok(layout)
    }

    fn request_passphrase(
        prompt: TString<'static>,
        _max_len: u32,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout =
            RootComponent::new(Frame::new(prompt, PassphraseEntry::new()).with_title_centered());
        Ok(layout)
    }

    fn select_menu(
        items: heapless::Vec<TString<'static>, MAX_MENU_ITEMS>,
        current: usize,
        _cancel: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        // Returning the index of the selected menu item
        let layout = RootComponent::new(
            SimpleChoice::new(items, ChoiceControls::Cancellable, TR::buttons__view.into())
                .with_initial_page_counter(current)
                .with_show_incomplete()
                .with_return_index()
                .with_ignore_cancelled(),
        );
        Ok(layout)
    }

    fn select_word(
        _title: TString<'static>,
        description: TString<'static>,
        words: [TString<'static>; MAX_WORD_QUIZ_ITEMS],
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let words: Vec<TString<'static>, 5> = Vec::from_iter(words);
        // Returning the index of the selected word, not the word itself
        let layout = RootComponent::new(
            Frame::new(
                description,
                SimpleChoice::new(words, ChoiceControls::Carousel, TR::buttons__select.into())
                    .with_show_incomplete()
                    .with_return_index(),
            )
            .with_title_centered(),
        );
        Ok(layout)
    }

    fn select_word_count(recovery_type: RecoveryType) -> Result<impl LayoutMaybeTrace, Error> {
        let title: TString = TR::word_count__title.into();
        let choices: Vec<TString<'static>, 5> = {
            let nums: &[&str] = if matches!(recovery_type, RecoveryType::UnlockRepeatedBackup) {
                &["20", "33"]
            } else {
                &["12", "18", "20", "24", "33"]
            };
            nums.iter().map(|&num| num.into()).collect()
        };
        let layout = RootComponent::new(
            Frame::new(
                title,
                SimpleChoice::new(
                    choices,
                    ChoiceControls::Cancellable,
                    TR::buttons__select.into(),
                ),
            )
            .with_title_centered(),
        );
        Ok(layout)
    }

    fn set_brightness(_current_brightness: Option<u8>) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_address_details(
        _qr_title: TString<'static>,
        address: TString<'static>,
        case_sensitive: bool,
        _details_title: TString<'static>,
        account: Option<TString<'static>>,
        path: Option<TString<'static>>,
        xpubs: Obj,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut ad = AddressDetails::new(address, case_sensitive, account, path)?;

        for i in IterBuf::new().try_iterate(xpubs)? {
            let [xtitle, text]: [StrBuffer; 2] = util::iter_into_array(i)?;
            ad.add_xpub(xtitle, text)?;
        }

        let layout = RootComponent::new(ad);
        Ok(layout)
    }

    fn show_checklist(
        _title: TString<'static>,
        button: TString<'static>,
        active: usize,
        items: [TString<'static>; MAX_CHECKLIST_ITEMS],
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut paragraphs = ParagraphVecLong::new();
        for (i, item) in items.into_iter().enumerate() {
            let style = match i.cmp(&active) {
                Ordering::Less => &theme::TEXT_NORMAL,
                Ordering::Equal => &theme::TEXT_BOLD,
                Ordering::Greater => &theme::TEXT_NORMAL,
            };
            paragraphs.add(Paragraph::new(style, item));
        }
        let confirm_btn = Some(ButtonDetails::text(button));

        let layout = RootComponent::new(
            ButtonPage::new(
                Checklist::from_paragraphs(
                    theme::ICON_ARROW_RIGHT_FAT,
                    theme::ICON_TICK_FAT,
                    active,
                    paragraphs
                        .into_paragraphs()
                        .with_spacing(theme::CHECKLIST_SPACING),
                )
                .with_check_width(theme::CHECKLIST_CHECK_WIDTH)
                .with_current_offset(theme::CHECKLIST_CURRENT_OFFSET),
                theme::BG,
            )
            .with_confirm_btn(confirm_btn),
        );
        Ok(layout)
    }

    fn show_danger(
        title: TString<'static>,
        description: TString<'static>,
        _value: TString<'static>,
        _menu_title: Option<TString<'static>>,
        _verb_cancel: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let get_page = move |page_index| {
            assert!(page_index == 0);

            let btn_layout = ButtonLayout::cancel_none_text(TR::buttons__continue.into());
            let btn_actions = ButtonActions::cancel_none_confirm();
            let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
            ops.add_alignment(geometry::Alignment::Center);
            if !title.is_empty() {
                ops.add_text(title, fonts::FONT_BOLD_UPPER);
                if !description.is_empty() {
                    ops.add_newline();
                }
            }
            if !description.is_empty() {
                ops.add_text(description, fonts::FONT_NORMAL);
            }
            let formatted = FormattedText::new(ops).vertically_centered();
            Page::new(btn_layout, btn_actions, formatted)
        };
        let pages = FlowPages::new(get_page, 1);
        let layout = RootComponent::new(Flow::new(pages));
        Ok(layout)
    }

    fn show_error(
        _title: TString<'static>,
        _button: TString<'static>,
        _description: TString<'static>,
        _allow_cancel: bool,
        _time_ms: u32,
    ) -> Result<Gc<LayoutObj>, Error> {
        Err::<Gc<LayoutObj>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_group_share_success(
        lines: [TString<'static>; MAX_GROUP_SHARE_LINES],
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let paragraphs = Paragraphs::new([
            Paragraph::new(&theme::TEXT_MONO, lines[0]),
            Paragraph::new(&theme::TEXT_BOLD, lines[1]),
            Paragraph::new(&theme::TEXT_MONO, lines[2]),
            Paragraph::new(&theme::TEXT_BOLD, lines[3]),
        ]);
        content_in_button_page(
            "".into(),
            paragraphs,
            TR::buttons__continue.into(),
            None,
            false,
        )
    }

    fn show_homescreen(
        label: TString<'static>,
        notification: Option<TString<'static>>,
        notification_level: u8,
        lockable: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let notification = notification.map(|w| (w, notification_level));
        let loader_description = lockable.then_some("Locking the device...".into());
        let layout = RootComponent::new(Homescreen::new(label, notification, loader_description));
        Ok(layout)
    }

    fn show_device_menu(
        _failed_backup: bool,
        _firmware_version: TString<'static>,
        _device_name: TString<'static>,
        _paired_devices: Vec<TString<'static>, 1>,
        _auto_lock_delay: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(Error::ValueError(
            c"show_device_menu not supported",
        ))
    }

    fn show_pairing_device_name(
        _device_name: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(Error::ValueError(
            c"show_pairing_device_name not supported",
        ))
    }

    #[cfg(feature = "ble")]
    fn show_ble_pairing_code(
        _title: TString<'static>,
        _description: TString<'static>,
        _code: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(Error::ValueError(
            c"show_ble_pairing_code not supported",
        ))
    }

    fn show_thp_pairing_code(
        _title: TString<'static>,
        _description: TString<'static>,
        _code: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(Error::ValueError(
            c"show_thp_pairing_code not supported",
        ))
    }

    fn show_info(
        title: TString<'static>,
        description: TString<'static>,
        _button: TString<'static>,
        time_ms: u32,
    ) -> Result<Gc<LayoutObj>, Error> {
        let content = Frame::new(
            title,
            Paragraphs::new([Paragraph::new(&theme::TEXT_NORMAL, description)]),
        );
        let obj = if time_ms == 0 {
            // No timer, used when we only want to draw the dialog once and
            // then throw away the layout object.
            LayoutObj::new(content)?
        } else {
            // Timeout.
            let timeout = Timeout::new(time_ms);
            LayoutObj::new((timeout, content.map(|_| None)))?
        };
        Ok(obj)
    }

    fn show_info_with_cancel(
        _title: TString<'static>,
        _items: Obj,
        _horizontal: bool,
        _chunkify: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_lockscreen(
        label: TString<'static>,
        bootscreen: bool,
        coinjoin_authorized: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let layout = RootComponent::new(Lockscreen::new(label, bootscreen, coinjoin_authorized));
        Ok(layout)
    }

    fn show_mismatch(title: TString<'static>) -> Result<impl LayoutMaybeTrace, Error> {
        let get_page = move |page_index| {
            assert!(page_index == 0);

            let btn_layout = ButtonLayout::arrow_none_text(TR::buttons__quit.into());
            let btn_actions = ButtonActions::cancel_none_confirm();
            let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
            ops.add_text(title, fonts::FONT_BOLD_UPPER)
                .add_newline()
                .add_newline_half()
                .add_text(TR::addr_mismatch__contact_support_at, fonts::FONT_NORMAL)
                .add_newline()
                .add_text(TR::addr_mismatch__support_url, fonts::FONT_BOLD);
            let formatted = FormattedText::new(ops);
            Page::new(btn_layout, btn_actions, formatted)
        };
        let pages = FlowPages::new(get_page, 1);

        let obj = RootComponent::new(Flow::new(pages));
        Ok(obj)
    }

    fn show_progress(
        description: TString<'static>,
        indeterminate: bool,
        title: Option<TString<'static>>,
        _danger: bool,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut progress = Progress::new(indeterminate, description);
        if let Some(title) = title {
            progress = progress.with_title(title);
        };

        let layout = RootComponent::new(progress);
        Ok(layout)
    }

    fn show_progress_coinjoin(
        title: TString<'static>,
        indeterminate: bool,
        time_ms: u32,
        skip_first_paint: bool,
    ) -> Result<Gc<LayoutObj>, Error> {
        let progress = CoinJoinProgress::new(title, indeterminate);
        let obj = if time_ms > 0 && indeterminate {
            let timeout = Timeout::new(time_ms);
            LayoutObj::new((timeout, progress.map(|_msg| None)))?
        } else {
            LayoutObj::new(progress)?
        };
        if skip_first_paint {
            obj.skip_first_paint();
        }
        Ok(obj)
    }

    fn show_properties(
        title: TString<'static>,
        value: Obj,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let mut paragraphs = ParagraphVecLong::new();
        if Obj::is_str(value) {
            // Display string value using monospace font
            add_paragraphs(&mut paragraphs, None, Some(value.try_into()?), true);
        } else {
            for para in IterBuf::new().try_iterate(value)? {
                let [key, value]: [Obj; 2] = util::iter_into_array(para)?;
                add_paragraphs(
                    &mut paragraphs,
                    key.try_into_option()?,
                    value.try_into_option()?,
                    false,
                );
            }
        }

        let page = ButtonPage::new(paragraphs.into_paragraphs(), theme::BG)
            .with_back_btn(Some(ButtonDetails::left_arrow_icon()))
            .with_next_btn(Some(ButtonDetails::right_arrow_icon()))
            .with_cancel_btn(Some(ButtonDetails::cancel_icon()))
            .with_confirm_btn(None);

        let mut frame = ScrollableFrame::new(page);
        if !title.is_empty() {
            frame = frame.with_title(title);
        }

        Ok(RootComponent::new(frame))
    }

    fn show_share_words(
        words: heapless::Vec<TString<'static>, 33>,
        _title: Option<TString<'static>>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        let cancel_btn = Some(ButtonDetails::up_arrow_icon());
        let confirm_btn =
            Some(ButtonDetails::text(TR::buttons__hold_to_confirm.into()).with_default_duration());

        let layout = RootComponent::new(
            ButtonPage::new(ShareWords::new(words), theme::BG)
                .with_cancel_btn(cancel_btn)
                .with_confirm_btn(confirm_btn),
        );
        Ok(layout)
    }

    fn show_share_words_extended(
        _words: heapless::Vec<TString<'static>, 33>,
        _subtitle: Option<TString<'static>>,
        _instructions: Obj,
        _instructions_verb: Option<TString<'static>>,
        _text_footer: Option<TString<'static>>,
        _text_confirm: TString<'static>,
        _text_check: TString<'static>,
    ) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_remaining_shares(_pages_iterable: Obj) -> Result<impl LayoutMaybeTrace, Error> {
        Err::<RootComponent<Empty, ModelUI>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_simple(
        text: TString<'static>,
        _title: Option<TString<'static>>,
        _button: Option<TString<'static>>,
    ) -> Result<Gc<LayoutObj>, Error> {
        let paragraph = Paragraph::new(&theme::TEXT_NORMAL, text).centered();
        let content = Paragraphs::new([paragraph]);
        let obj = LayoutObj::new(content)?;
        Ok(obj)
    }

    fn show_success(
        _title: TString<'static>,
        _button: TString<'static>,
        _description: TString<'static>,
        _allow_cancel: bool,
        _time_ms: u32,
    ) -> Result<Gc<LayoutObj>, Error> {
        Err::<Gc<LayoutObj>, Error>(ERROR_NOT_IMPLEMENTED)
    }

    fn show_wait_text(text: TString<'static>) -> Result<impl LayoutMaybeTrace, Error> {
        let layout =
            RootComponent::new(Connect::new(text, fonts::FONT_NORMAL, theme::FG, theme::BG));
        Ok(layout)
    }

    fn show_warning(
        _title: TString<'static>,
        button: TString<'static>,
        value: TString<'static>,
        description: TString<'static>,
        _allow_cancel: bool,
        _danger: bool,
    ) -> Result<Gc<LayoutObj>, Error> {
        let get_page = move |page_index| {
            assert!(page_index == 0);

            let btn_layout = ButtonLayout::none_armed_none(button);
            let btn_actions = ButtonActions::none_confirm_none();
            let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
            ops.add_alignment(geometry::Alignment::Center);
            if !value.is_empty() {
                ops.add_text(value, fonts::FONT_BOLD_UPPER);
                if !description.is_empty() {
                    ops.add_newline();
                }
            }
            if !description.is_empty() {
                ops.add_text(description, fonts::FONT_NORMAL);
            }
            let formatted = FormattedText::new(ops).vertically_centered();
            Page::new(btn_layout, btn_actions, formatted)
        };
        let pages = FlowPages::new(get_page, 1);
        let obj = LayoutObj::new(Flow::new(pages))?;
        Ok(obj)
    }

    fn tutorial() -> Result<impl LayoutMaybeTrace, Error> {
        const PAGE_COUNT: usize = 7;

        let get_page = move |page_index| {
            // Lazy-loaded list of screens to show, with custom content,
            // buttons and actions triggered by these buttons.
            // Cancelling the first screen will point to the last one,
            // which asks for confirmation whether user wants to
            // really cancel the tutorial.
            match page_index {
                // title, text, btn_layout, btn_actions
                0 => tutorial_screen(
                    TR::tutorial__title_hello.into(),
                    TR::tutorial__welcome_press_right,
                    ButtonLayout::cancel_none_arrow(),
                    ButtonActions::last_none_next(),
                ),
                1 => tutorial_screen(
                    "".into(),
                    TR::tutorial__use_trezor,
                    ButtonLayout::arrow_none_arrow(),
                    ButtonActions::prev_none_next(),
                ),
                2 => tutorial_screen(
                    TR::buttons__hold_to_confirm.into(),
                    TR::tutorial__press_and_hold,
                    ButtonLayout::arrow_none_htc(TR::buttons__hold_to_confirm.into()),
                    ButtonActions::prev_none_next(),
                ),
                3 => tutorial_screen(
                    TR::tutorial__title_screen_scroll.into(),
                    TR::tutorial__scroll_down,
                    ButtonLayout::arrow_none_text(TR::buttons__continue.into()),
                    ButtonActions::prev_none_next(),
                ),
                4 => tutorial_screen(
                    TR::buttons__confirm.into(),
                    TR::tutorial__middle_click,
                    ButtonLayout::none_armed_none(TR::buttons__confirm.into()),
                    ButtonActions::none_next_none(),
                ),
                5 => tutorial_screen(
                    TR::tutorial__title_tutorial_complete.into(),
                    TR::tutorial__ready_to_use,
                    ButtonLayout::text_none_text(
                        TR::buttons__again.into(),
                        TR::buttons__continue.into(),
                    ),
                    ButtonActions::beginning_none_confirm(),
                ),
                6 => tutorial_screen(
                    TR::tutorial__title_skip.into(),
                    TR::tutorial__sure_you_want_skip,
                    ButtonLayout::arrow_none_text(TR::buttons__skip.into()),
                    ButtonActions::beginning_none_cancel(),
                ),
                _ => unreachable!(),
            }
        };

        let pages = FlowPages::new(get_page, PAGE_COUNT);

        // Setting the ignore-second-button to mimic all the Choice pages, to teach user
        // that they should really press both buttons at the same time to achieve
        // middle-click.
        let layout = RootComponent::new(
            Flow::new(pages)
                .with_scrollbar(false)
                .with_ignore_second_button_ms(constant::IGNORE_OTHER_BTN_MS),
        );
        Ok(layout)
    }
}

/// Function to create and call a `ButtonPage` dialog based on paginable content
/// (e.g. `Paragraphs` or `FormattedText`).
/// Has optional title (supply empty `TString` for that) and hold-to-confirm
/// functionality.
fn content_in_button_page<T: Component + Paginate + MaybeTrace + 'static>(
    title: TString<'static>,
    content: T,
    verb: TString<'static>,
    verb_cancel: Option<TString<'static>>,
    hold: bool,
) -> Result<impl LayoutMaybeTrace, Error> {
    // Left button - icon, text or nothing.
    let cancel_btn = verb_cancel.map(ButtonDetails::from_text_possible_icon);

    // Right button - down arrow, text or nothing.
    // Optional HoldToConfirm
    let mut confirm_btn = if !verb.is_empty() {
        if verb == TString::Str("V") {
            Some(ButtonDetails::down_arrow_icon_wide())
        } else {
            Some(ButtonDetails::text(verb))
        }
    } else {
        None
    };
    if hold {
        confirm_btn = confirm_btn.map(|btn| btn.with_default_duration());
    }

    let content = ButtonPage::new(content, theme::BG)
        .with_cancel_btn(cancel_btn)
        .with_confirm_btn(confirm_btn);

    let mut frame = ScrollableFrame::new(content);
    if !title.is_empty() {
        frame = frame.with_title(title);
    }

    Ok(RootComponent::new(frame))
}

/// General pattern of most tutorial screens.
/// (title, text, btn_layout, btn_actions, text_y_offset)
fn tutorial_screen(
    title: TString<'static>,
    text: TR,
    btn_layout: ButtonLayout,
    btn_actions: ButtonActions,
) -> Page {
    let mut ops = OpTextLayout::new(theme::TEXT_NORMAL);
    ops.add_text(text, fonts::FONT_NORMAL);
    let formatted = FormattedText::new(ops).vertically_centered();
    Page::new(btn_layout, btn_actions, formatted).with_title(title)
}

fn add_paragraphs<'a>(
    paragraphs: &mut ParagraphVecLong<'a>,
    key: Option<TString<'a>>,
    value: Option<TString<'a>>,
    is_data: bool,
) {
    if let Some(key) = key {
        if value.is_some() {
            // Decreasing the margin between key and value (default is 5 px, we use 2 px)
            // (this enables 4 lines - 2 key:value pairs - on the same screen)
            paragraphs.add(
                Paragraph::new(&theme::TEXT_BOLD, key)
                    .no_break()
                    .with_bottom_padding(2),
            );
        } else {
            paragraphs.add(Paragraph::new(&theme::TEXT_BOLD, key));
        }
    }
    if let Some(value) = value {
        let style = if is_data {
            &theme::TEXT_MONO_DATA
        } else {
            &theme::TEXT_MONO
        };
        paragraphs.add(Paragraph::new(style, value));
    }
}

fn parse_properties(items: Obj) -> Result<ParagraphVecLong<'static>, Error> {
    let mut paragraphs = ParagraphVecLong::new();

    for para in IterBuf::new().try_iterate(items)? {
        let [key, value, is_data]: [Obj; 3] = util::iter_into_array(para)?;
        add_paragraphs(
            &mut paragraphs,
            key.try_into_option()?,
            value.try_into_option()?,
            is_data.try_into()?,
        );
    }

    Ok(paragraphs)
}
