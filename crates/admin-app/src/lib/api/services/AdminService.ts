/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateMobDropRateRequest } from '../models/CreateMobDropRateRequest';
import type { CreateMobDropRateResponse } from '../models/CreateMobDropRateResponse';
import type { ItemDetailsResponse } from '../models/ItemDetailsResponse';
import type { ListItemsResponse } from '../models/ListItemsResponse';
import type { ListMobsResponse } from '../models/ListMobsResponse';
import type { MobDetailsResponse } from '../models/MobDetailsResponse';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class AdminService {
    /**
     * @returns ListItemsResponse Items listed
     * @throws ApiError
     */
    public static listItems({
        page,
        limit,
        _class,
        rarity,
        equipmentSlot,
        inventoryType,
        levelMin,
        levelMax,
        search,
    }: {
        /**
         * Page number (default: 1)
         */
        page?: number,
        /**
         * Items per page (default: 20)
         */
        limit?: number,
        /**
         * Filter by class
         */
        _class?: string,
        /**
         * Filter by rarity
         */
        rarity?: string,
        /**
         * Filter by equipment slot
         */
        equipmentSlot?: string,
        /**
         * Filter by inventory type
         */
        inventoryType?: string,
        /**
         * Minimum level requirement
         */
        levelMin?: number,
        /**
         * Maximum level requirement
         */
        levelMax?: number,
        /**
         * Search by name
         */
        search?: string,
    }): CancelablePromise<ListItemsResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/items',
            query: {
                'page': page,
                'limit': limit,
                'class': _class,
                'rarity': rarity,
                'equipmentSlot': equipmentSlot,
                'inventoryType': inventoryType,
                'levelMin': levelMin,
                'levelMax': levelMax,
                'search': search,
            },
            errors: {
                401: `Unauthorized`,
                403: `Forbidden`,
            },
        });
    }
    /**
     * @returns ItemDetailsResponse Item found
     * @throws ApiError
     */
    public static getItem({
        slug,
    }: {
        /**
         * Item slug
         */
        slug: string,
    }): CancelablePromise<ItemDetailsResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/items/{slug}',
            path: {
                'slug': slug,
            },
            errors: {
                401: `Unauthorized`,
                403: `Forbidden`,
                404: `Not found`,
            },
        });
    }
    /**
     * @returns CreateMobDropRateResponse Mob drop rate created
     * @throws ApiError
     */
    public static createMobDropRate({
        requestBody,
    }: {
        requestBody: CreateMobDropRateRequest,
    }): CancelablePromise<CreateMobDropRateResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/mob-drop-rates',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @returns ListMobsResponse Mobs listed
     * @throws ApiError
     */
    public static listMobs({
        page,
        limit,
        mobType,
        search,
    }: {
        /**
         * Page number (default: 1)
         */
        page?: number,
        /**
         * Mobs per page (default: 20)
         */
        limit?: number,
        /**
         * Filter by mob type
         */
        mobType?: string,
        /**
         * Search by name
         */
        search?: string,
    }): CancelablePromise<ListMobsResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/mobs',
            query: {
                'page': page,
                'limit': limit,
                'mobType': mobType,
                'search': search,
            },
            errors: {
                401: `Unauthorized`,
                403: `Forbidden`,
            },
        });
    }
    /**
     * @returns MobDetailsResponse Mob found
     * @throws ApiError
     */
    public static getMob({
        slug,
    }: {
        /**
         * Mob slug
         */
        slug: string,
    }): CancelablePromise<MobDetailsResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/mobs/{slug}',
            path: {
                'slug': slug,
            },
            errors: {
                401: `Unauthorized`,
                403: `Forbidden`,
                404: `Not found`,
            },
        });
    }
}
