use leptos::prelude::*;

use crate::seo::Seo;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ContactInput {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[server]
pub async fn send_contact(input: ContactInput) -> Result<(), ServerFnError> {
    let name = input.name.trim().to_string();
    let email = input.email.trim().to_string();
    let subject = input.subject.trim().to_string();
    let message = input.message.trim().to_string();

    if name.len() < 2 {
        return Err(ServerFnError::new("Nama minimal 2 karakter."));
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(ServerFnError::new("Format email tidak valid."));
    }
    if message.len() < 10 {
        return Err(ServerFnError::new("Pesan minimal 10 karakter."));
    }

    let api_key = std::env::var("SMTP_KEY")
        .map_err(|_| ServerFnError::new("Email service tidak terkonfigurasi."))?;
    let email_from = std::env::var("EMAIL_FROM")
        .map_err(|_| ServerFnError::new("Email service tidak terkonfigurasi."))?;
    let email_to = std::env::var("EMAIL_TO")
        .map_err(|_| ServerFnError::new("Email service tidak terkonfigurasi."))?;

    let subject_line = if subject.is_empty() {
        format!("[Portfolio] Pesan dari {name}")
    } else {
        format!("[Portfolio] {subject}")
    };

    let body = format!(
        "Nama    : {name}\nEmail   : {email}\n\nPesan:\n{message}"
    );

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&api_key)
        .json(&serde_json::json!({
            "from": email_from,
            "to": [email_to],
            "reply_to": email,
            "subject": subject_line,
            "text": body,
        }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Gagal mengirim: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Resend error ({status}): {text}")));
    }

    Ok(())
}

#[allow(non_snake_case)]
#[component]
pub fn ContactPage() -> impl IntoView {
    let action = ServerAction::<SendContact>::new();

    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let subject = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());

    let sent = move || {
        action
            .value()
            .get()
            .as_ref()
            .map(|r| r.is_ok())
            .unwrap_or(false)
    };

    let error_msg = move || {
        action.value().get().as_ref().and_then(|r| match r {
            Err(e) => Some(e.to_string()),
            Ok(_) => None,
        })
    };

    view! {
        <Seo
            title="Contact — Feri Irawansyah"
            description="Hubungi Feri Irawansyah — Rust programmer dan Principal Engineer. Tersedia untuk kolaborasi, konsultasi, dan project baru."
            path="/contact"
        />
        <div class="py-4">
            <div class="max-w-2xl mx-auto px-6 py-20">

                // Header
                <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                    "Contact"
                </span>
                <h1 class="text-[2rem] font-extrabold text-fg mb-3">"Hubungi Saya"</h1>
                <p class="text-muted leading-relaxed mb-10">
                    "Tertarik untuk berkolaborasi, ada pertanyaan, atau sekadar ingin ngobrol soal Rust dan engineering? Kirim pesan di bawah ini."
                </p>

                {move || if sent() {
                    view! {
                        <div class="bg-teal-500/10 border border-teal-500/40 rounded-xl p-6 text-center">
                            <div class="w-12 h-12 rounded-full bg-teal-500/20 flex items-center justify-center mx-auto mb-4">
                                <i class="bi bi-check-lg text-teal-500 text-xl"></i>
                            </div>
                            <h2 class="text-lg font-bold text-fg mb-2">"Pesan Terkirim!"</h2>
                            <p class="text-muted text-sm">"Terima kasih! Saya akan membalas secepatnya."</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="bg-surface border border-line rounded-2xl p-6 md:p-8">

                            // Error
                            {move || error_msg().map(|e| view! {
                                <div class="mb-5 bg-red-500/10 border border-red-500/30 rounded-lg px-4 py-3 text-red-400 text-sm flex items-start gap-2">
                                    <i class="bi bi-exclamation-circle shrink-0 mt-0.5"></i>
                                    <span>{e}</span>
                                </div>
                            })}

                            <ActionForm action=action>
                                // Hidden fields buat pass ke ContactInput
                                <input type="hidden" name="input[name]" prop:value=move || name.get()/>
                                <input type="hidden" name="input[email]" prop:value=move || email.get()/>
                                <input type="hidden" name="input[subject]" prop:value=move || subject.get()/>
                                <input type="hidden" name="input[message]" prop:value=move || message.get()/>

                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                                    <div>
                                        <label class="block text-xs font-semibold text-muted uppercase tracking-wide mb-1.5">
                                            "Nama " <span class="text-red-400">"*"</span>
                                        </label>
                                        <input
                                            type="text"
                                            name="name_field"
                                            required
                                            placeholder="Nama kamu"
                                            on:input=move |e| name.set(event_target_value(&e))
                                            class="w-full bg-bg border border-line rounded-lg px-4 py-2.5 text-sm text-fg placeholder:text-muted/50 focus:outline-none focus:border-teal-500 transition-colors"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-muted uppercase tracking-wide mb-1.5">
                                            "Email " <span class="text-red-400">"*"</span>
                                        </label>
                                        <input
                                            type="email"
                                            name="email_field"
                                            required
                                            placeholder="email@kamu.com"
                                            on:input=move |e| email.set(event_target_value(&e))
                                            class="w-full bg-bg border border-line rounded-lg px-4 py-2.5 text-sm text-fg placeholder:text-muted/50 focus:outline-none focus:border-teal-500 transition-colors"
                                        />
                                    </div>
                                </div>

                                <div class="mb-4">
                                    <label class="block text-xs font-semibold text-muted uppercase tracking-wide mb-1.5">
                                        "Subjek"
                                    </label>
                                    <input
                                        type="text"
                                        name="subject_field"
                                        placeholder="Topik pesan (opsional)"
                                        on:input=move |e| subject.set(event_target_value(&e))
                                        class="w-full bg-bg border border-line rounded-lg px-4 py-2.5 text-sm text-fg placeholder:text-muted/50 focus:outline-none focus:border-teal-500 transition-colors"
                                    />
                                </div>

                                <div class="mb-6">
                                    <label class="block text-xs font-semibold text-muted uppercase tracking-wide mb-1.5">
                                        "Pesan " <span class="text-red-400">"*"</span>
                                    </label>
                                    <textarea
                                        name="message_field"
                                        required
                                        rows="5"
                                        placeholder="Tulis pesanmu di sini..."
                                        on:input=move |e| message.set(event_target_value(&e))
                                        class="w-full bg-bg border border-line rounded-lg px-4 py-2.5 text-sm text-fg placeholder:text-muted/50 focus:outline-none focus:border-teal-500 transition-colors resize-none"
                                    ></textarea>
                                </div>

                                <button
                                    type="submit"
                                    disabled=move || action.pending().get()
                                    class="w-full flex items-center justify-center gap-2 px-6 py-3 bg-teal-600 hover:bg-teal-500 text-white rounded-lg text-sm font-semibold transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {move || if action.pending().get() {
                                        view! {
                                            <span class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></span>
                                            "Mengirim..."
                                        }.into_any()
                                    } else {
                                        view! {
                                            <i class="bi bi-send"></i>
                                            "Kirim Pesan"
                                        }.into_any()
                                    }}
                                </button>
                            </ActionForm>
                        </div>

                        // Alt contact
                        <div class="mt-8 flex flex-col sm:flex-row gap-4">
                            <a href="https://wa.me/6282323443535" target="_blank"
                                class="flex items-center gap-3 px-5 py-3 border border-line rounded-xl text-sm text-muted hover:border-teal-500 hover:text-fg transition-colors no-underline">
                                <i class="bi bi-whatsapp text-green-400"></i>
                                "WhatsApp"
                            </a>
                            <a href="https://github.com/feri-irawansyah" target="_blank"
                                class="flex items-center gap-3 px-5 py-3 border border-line rounded-xl text-sm text-muted hover:border-teal-500 hover:text-fg transition-colors no-underline">
                                <i class="bi bi-github"></i>
                                "GitHub"
                            </a>
                        </div>
                    }.into_any()
                }}

            </div>
        </div>
    }
}
