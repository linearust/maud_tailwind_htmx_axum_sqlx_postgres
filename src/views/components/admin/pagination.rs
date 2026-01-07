use maud::{html, Markup};
use crate::paths;

pub fn pagination(base_path: &str, current_page: i64, total_pages: i64, has_prev: bool, has_next: bool) -> Markup {
    if total_pages <= 1 {
        return html! {};
    }

    html! {
        div class="flex items-center justify-center gap-2 mt-4" {
            @if has_prev {
                a href=(paths::with_page(base_path, current_page - 1))
                    class="text-indigo-600 hover:underline"
                {
                    "←"
                }
            } @else {
                span class="text-gray-400" { "←" }
            }

            @for page in page_numbers(current_page, total_pages) {
                @match page {
                    PageNumber::Page(page_number) => {
                        @if page_number == current_page {
                            span class="px-2" { (page_number) }
                        } @else {
                            a href=(paths::with_page(base_path, page_number))
                                class="px-2 text-indigo-600 hover:underline"
                            {
                                (page_number)
                            }
                        }
                    }
                    PageNumber::Ellipsis => {
                        span class="px-2" { "..." }
                    }
                }
            }

            @if has_next {
                a href=(paths::with_page(base_path, current_page + 1))
                    class="text-indigo-600 hover:underline"
                {
                    "→"
                }
            } @else {
                span class="text-gray-400" { "→" }
            }
        }
    }
}

enum PageNumber {
    Page(i64),
    Ellipsis,
}

fn page_numbers(current: i64, total: i64) -> Vec<PageNumber> {
    let mut pages = Vec::new();

    if total <= 7 {
        for page_number in 1..=total {
            pages.push(PageNumber::Page(page_number));
        }
        return pages;
    }

    pages.push(PageNumber::Page(1));

    if current > 3 {
        pages.push(PageNumber::Ellipsis);
    }

    let start = (current - 1).max(2);
    let end = (current + 1).min(total - 1);

    for page_number in start..=end {
        pages.push(PageNumber::Page(page_number));
    }

    if current < total - 2 {
        pages.push(PageNumber::Ellipsis);
    }

    pages.push(PageNumber::Page(total));

    pages
}
