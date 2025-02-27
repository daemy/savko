<script lang="ts">
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import Check from "lucide-svelte/icons/check";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
    import * as Accordion from "$lib/components/ui/accordion/index.js";

    let notifs: { message: string; description: string }[] = [];
    let closeDialogOpen : boolean = false;
    const LOAD_COUNT = 5;

    async function deleteAll() {
        await invoke("wipe_all");
        notifs = [];
        closeDialogOpen = false;
    }

    async function copy() {
        toast.success("Copied to clipboard.", {
            // this is hardcoded date for now for testing UI, I will provide date from Rust later
            description: "",
            action: {
                label: "Dismiss",
                onClick: () => console.info("Undo")
            }
        })
    }

    async function load_last_n() {
        try {
            notifs = await invoke("load_last_n_entries", { n: LOAD_COUNT });
        } catch (error) {
            console.error("Error loading clipboard history:", error);
        }
    }

    onMount(async () => {
        await load_last_n();
        // Create a new Channel instance
        const onEvent = new Channel<string>();
        onEvent.onmessage = (message: string) => {
            console.log("Clipboard updated:", message);
            // Append new clipboard text into our notifications array
            notifs = [{ message, description: "Copied to clipboard" }, ...notifs];
        };

        try {
            await invoke("init", { onEvent });
        } catch (error) {
            console.error("Error initializing clipboard listener:", error);
        }
    });
</script>

<Card.Root class="w-full">
    <Card.Header>
        <Card.Title>Clipboard History</Card.Title>
        <Card.Description>You have {notifs.length} saved entries.</Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4">
        <Accordion.Root type="single" class="w-full sm:max-w-[70%]">
        {#each notifs as notification, idx}
                <Accordion.Item>
                    <Accordion.Trigger>{notification.message.slice(0, 20)}</Accordion.Trigger>
                    <Accordion.Content onclick={copy}
                    >{notification.message}</Accordion.Content
                    >
                </Accordion.Item>
        {/each}
        </Accordion.Root>
    </Card.Content>
    <AlertDialog.Root bind:open={closeDialogOpen}>
        <AlertDialog.Trigger onclick={() => closeDialogOpen = true}>
            <Card.Footer class="bottom-0">
                <Button class="w-full bottom-0">
                    <Check /> Delete all
                </Button>
            </Card.Footer>
        </AlertDialog.Trigger>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
                <AlertDialog.Description>
                    This action cannot be undone. This will permanently delete your saved clipboards
                    and remove your data from your local computer.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel oncancel={() => closeDialogOpen = false}>Cancel</AlertDialog.Cancel>
                <AlertDialog.Action onclick={deleteAll}>Continue</AlertDialog.Action>
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>
</Card.Root>
