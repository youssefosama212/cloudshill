<script lang="ts">
	import type iconsJson from '$lib/icons/icons.json';
	import Icon from '$lib/icons/Icon.svelte';
	import { createEventDispatcher } from 'svelte';

	export let id: string | undefined = undefined;
	export let icon: keyof typeof iconsJson | undefined = undefined;
	export let value: string | undefined = undefined;
	export let placeholder: string | undefined = undefined;
	export let iconPosition: 'left' | 'right' = 'left';
	export let disabled = false;
	export let readonly = false;
	export let required = false;
	export let password = false;

	const dispatch = createEventDispatcher<{ input: string; change: string }>();
</script>

<div class="textbox">
	<div
		class="textbox__input-wrap"
		class:textbox__left-orient={icon && iconPosition == 'left'}
		class:textbox__right-orient={icon && iconPosition == 'right'}
		class:disabled
	>
		{#if icon}
			<div class="textbox__icon">
				<Icon name={icon} />
			</div>
		{/if}
		{#if password}
			<input
				{id}
				type="password"
				class="textbox__input text-base-13"
				{placeholder}
				{readonly}
				{required}
				bind:value
				on:input={(e) => dispatch('input', e.currentTarget.value)}
				on:change={(e) => dispatch('change', e.currentTarget.value)}
			/>
		{:else}
			<input
				{id}
				class="textbox__input text-base-13"
				{placeholder}
				{readonly}
				{required}
				bind:value
				on:input={(e) => dispatch('input', e.currentTarget.value)}
				on:change={(e) => dispatch('change', e.currentTarget.value)}
			/>
		{/if}
	</div>
</div>

<style lang="postcss">
	.textbox {
		position: relative;
		display: flex;
		flex-direction: column;
	}

	.textbox__input-wrap {
		position: relative;
	}

	.textbox__input {
		flex-grow: 1;
		height: var(--space-32);
		width: 100%;
		padding: var(--space-4) var(--space-12);
		color: var(--clr-theme-scale-ntrl-0);
		background-color: var(--clr-theme-container-light);
		border: 1px solid var(--clr-theme-container-outline-light);
		border-radius: var(--radius-s);
		transition: border-color var(--transition-fast);

		&::placeholder {
			color: var(--clr-theme-scale-ntrl-50);
		}

		&:hover {
			border-color: var(--clr-theme-container-outline-pale);
		}

		&:focus {
			border-color: var(--clr-theme-container-outline-sub);
			outline: none;
		}

		&:invalid {
			border-color: var(--clr-theme-err-element);
		}

		&:disabled {
			color: var(--clr-theme-scale-ntrl-60);
			border-color: var(--clr-theme-err-element);
			background-color: var(--clr-theme-container-pale);
		}
	}

	.textbox__icon {
		pointer-events: none;
		position: absolute;
		top: 50%;
		color: var(--clr-theme-scale-ntrl-50);
		transform: translateY(-50%);
	}

	/* Modifiers */

	.textbox__left-orient {
		& .textbox__input {
			padding-left: calc(var(--space-32) + var(--space-2));
		}
		& .textbox__icon {
			left: var(--space-12);
		}
	}

	.textbox__right-orient {
		& .textbox__input {
			padding-right: calc(var(--space-32) + var(--space-2));
		}
		& .textbox__icon {
			right: var(--space-12);
		}
	}

	.disabled {
		& .textbox__input {
			color: var(--clr-theme-scale-ntrl-60);
			border-color: var(--clr-theme-scale-ntrl-70);
			background-color: var(--clr-theme-container-pale);
		}

		& .textbox__icon {
			color: var(--clr-theme-scale-ntrl-60);
		}
	}
</style>