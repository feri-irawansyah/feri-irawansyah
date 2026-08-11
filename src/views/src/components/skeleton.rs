//! Shimmer placeholders shown in `<Suspense fallback=...>` while a page's
//! data resource is still in flight, replacing plain "Loading..." text.
//! Each one is sized/shaped to match the real content it stands in for
//! (same principle as the admin `TableSkeleton`) so nothing jumps around
//! once the real data lands.
use leptos::prelude::*;

/// Shared shimmer primitive every skeleton below is built from.
fn bar(class: &str) -> impl IntoView {
    view! { <div class=format!("bg-line rounded-md animate-pulse {class}")></div> }
}

/// Stand-in for `NoteCard` rows on the `/notes` listing (and search results).
#[allow(non_snake_case)]
#[component]
pub fn NoteCardSkeleton(#[prop(default = 3)] count: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            {(0..count).map(|_| view! {
                <div class="flex flex-col sm:flex-row gap-4 sm:gap-5 items-start bg-surface border border-line rounded-2xl p-4 sm:p-5">
                    <div class="w-full h-44 sm:w-50 sm:h-32.5 rounded-lg shrink-0 bg-line animate-pulse"></div>
                    <div class="flex-1 min-w-0 w-full">
                        <div class="flex gap-3 items-center mb-2.5">
                            {bar("h-3 w-16")}
                            {bar("h-3 w-20")}
                        </div>
                        {bar("h-4 w-3/4 mb-2")}
                        {bar("h-3 w-full mb-1.5")}
                        {bar("h-3 w-2/3 mb-3")}
                        <div class="flex gap-1.5">
                            {bar("h-5 w-14 rounded-full")}
                            {bar("h-5 w-12 rounded-full")}
                        </div>
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}

/// Stand-in for the note/journey/laboratory hero image (title+meta baked
/// into the image overlay, so — unlike the other skeletons — there's no
/// separate text row to fake underneath it).
#[allow(non_snake_case)]
#[component]
pub fn NoteHeroSkeleton() -> impl IntoView {
    view! {
        <div class="w-full h-64 sm:h-80 lg:h-95 bg-line rounded-2xl animate-pulse mb-10"></div>
    }
}

/// Stand-in for a plain article header: optional icon chip + title bar,
/// used by pages whose header has no hero image (journey, laboratory detail).
#[allow(non_snake_case)]
#[component]
pub fn ArticleHeaderSkeleton(#[prop(default = false)] with_icon: bool) -> impl IntoView {
    view! {
        <div class="pb-8 border-b border-line mb-8">
            {with_icon.then(|| view! {
                <div class="w-10 h-10 rounded-xl bg-line animate-pulse mb-5"></div>
            })}
            <div class="h-8 w-2/3 bg-line rounded-md animate-pulse"></div>
        </div>
    }
}

/// Stand-in for rendered markdown body content (note/journey/laboratory
/// article text) while it's being fetched and processed.
#[allow(non_snake_case)]
#[component]
pub fn ContentLinesSkeleton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3">
            {bar("h-4 w-full")}
            {bar("h-4 w-full")}
            {bar("h-4 w-5/6")}
            {bar("h-4 w-full")}
            {bar("h-4 w-2/3")}
            {bar("h-40 w-full rounded-xl mt-2")}
            {bar("h-4 w-full mt-2")}
            {bar("h-4 w-4/5")}
            {bar("h-4 w-full")}
        </div>
    }
}

/// Stand-in for laboratory category listing rows (title + description, no
/// thumbnail).
#[allow(non_snake_case)]
#[component]
pub fn ListRowSkeleton(#[prop(default = 4)] count: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            {(0..count).map(|_| view! {
                <div class="flex flex-col gap-2 bg-surface border border-line rounded-2xl p-5">
                    {bar("h-4 w-1/2 mb-1")}
                    {bar("h-3 w-full")}
                    {bar("h-3 w-2/3")}
                </div>
            }).collect_view()}
        </div>
    }
}

/// Stand-in for portfolio project cards (image top, title/date row,
/// description, link).
#[allow(non_snake_case)]
#[component]
pub fn PortfolioCardSkeleton(#[prop(default = 6)] count: usize) -> impl IntoView {
    view! {
        <div class="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-6">
            {(0..count).map(|_| view! {
                <div class="bg-surface border border-line rounded-2xl overflow-hidden">
                    <div class="w-full h-47.5 bg-line animate-pulse"></div>
                    <div class="p-6">
                        <div class="flex items-center justify-between gap-3 mb-3">
                            {bar("h-4 w-1/2")}
                            {bar("h-3 w-14")}
                        </div>
                        {bar("h-3 w-full mb-1.5")}
                        {bar("h-3 w-2/3 mb-4")}
                        {bar("h-3 w-24")}
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}

/// Stand-in for tech-stack cards on `/skills` (icon square + title +
/// progress bar).
#[allow(non_snake_case)]
#[component]
pub fn SkillCardSkeleton(#[prop(default = 9)] count: usize) -> impl IntoView {
    view! {
        <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-5">
            {(0..count).map(|_| view! {
                <div class="bg-surface border border-line rounded-xl p-6 flex flex-col items-center gap-4">
                    <div class="w-14 h-14 rounded-xl bg-line animate-pulse"></div>
                    {bar("h-3 w-16")}
                    {bar("h-2 w-full rounded-full")}
                </div>
            }).collect_view()}
        </div>
    }
}

/// Stand-in for work-history timeline cards on `/experience`.
#[allow(non_snake_case)]
#[component]
pub fn TimelineCardSkeleton(#[prop(default = 3)] count: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            {(0..count).map(|_| view! {
                <div class="relative flex gap-3 sm:gap-6 pb-8">
                    <div class="w-4 h-4 rounded-full bg-line shrink-0 mt-1"></div>
                    <div class="flex-1 min-w-0 bg-surface border border-line rounded-2xl p-4 sm:p-6">
                        {bar("h-4 w-1/3 mb-2")}
                        {bar("h-3 w-1/4 mb-5")}
                        <div class="flex flex-col gap-2.5 pl-5 border-l-2 border-line ml-1">
                            {bar("h-3.5 w-1/2")}
                            {bar("h-3 w-full")}
                            {bar("h-3 w-2/3")}
                        </div>
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}

/// Stand-in for certification cards on `/experience` (icon square + title +
/// description + date).
#[allow(non_snake_case)]
#[component]
pub fn CertCardSkeleton(#[prop(default = 6)] count: usize) -> impl IntoView {
    view! {
        <div class="grid grid-cols-[repeat(auto-fill,minmax(260px,1fr))] gap-4">
            {(0..count).map(|_| view! {
                <div class="bg-surface border border-line rounded-2xl p-5 flex gap-4 items-start">
                    <div class="w-12 h-12 rounded-lg bg-line animate-pulse shrink-0"></div>
                    <div class="flex-1 min-w-0">
                        {bar("h-3.5 w-2/3 mb-2")}
                        {bar("h-3 w-full mb-1")}
                        {bar("h-3 w-1/3")}
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}
