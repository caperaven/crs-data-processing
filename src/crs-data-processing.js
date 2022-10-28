import {date_difference_str} from "./data_processing_date_time.js";

export class DataProcessingActions {
    static async perform(step, context, process, item) {
        await this[step.action]?.(step, context, process, item);
    }

    static async date_difference_str(step, context, process, item) {
        const date1 = await crs.process.getValue(step.args.date1, context, process, item);
        const date2 = await crs.process.getValue(step.args.date1, context, process, item);

        const result = date_difference_str(date1, date2);

        if (step.args.target != null) {
            await crs.process.setValue(step.args.target, result, context, process, item);
        }

        return result;
    }
}

crs.intent.data_processing = DataProcessingActions;