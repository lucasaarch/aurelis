/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateItemRequest } from '../models/CreateItemRequest';
import type { CreateItemResponse } from '../models/CreateItemResponse';
import type { CreateMobDropRateRequest } from '../models/CreateMobDropRateRequest';
import type { CreateMobDropRateResponse } from '../models/CreateMobDropRateResponse';
import type { CreateMobRequest } from '../models/CreateMobRequest';
import type { CreateMobResponse } from '../models/CreateMobResponse';
import type { GiveItemRequest } from '../models/GiveItemRequest';
import type { GiveItemResponse } from '../models/GiveItemResponse';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class AdminService {
    /**
     * @returns CreateItemResponse Item created
     * @throws ApiError
     */
    public static createItem({
        requestBody,
    }: {
        requestBody: CreateItemRequest,
    }): CancelablePromise<CreateItemResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/items',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                401: `Unauthorized`,
            },
        });
    }
    /**
     * @returns GiveItemResponse Item given
     * @throws ApiError
     */
    public static giveItem({
        requestBody,
    }: {
        requestBody: GiveItemRequest,
    }): CancelablePromise<GiveItemResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/items/give',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                401: `Unauthorized`,
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
     * @returns CreateMobResponse Mob created
     * @throws ApiError
     */
    public static createMob({
        requestBody,
    }: {
        requestBody: CreateMobRequest,
    }): CancelablePromise<CreateMobResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/mobs',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                401: `Unauthorized`,
            },
        });
    }
}
