import React, { ChangeEvent } from "react";
import { useUploadMutationMutation } from "./generated/graphql";
// import graphql from "babel-plugin-relay/macro";
// import { useMutation } from "react-relay";
// import { UploadMutation } from "./__generated__/UploadMutation.graphql";

function Upload() {
  const [res, exec] = useUploadMutationMutation();
  // const [commit] = useMutation<UploadMutation>(uploadMutation);
  console.log(res);
  return (
    <div>
      <form>
        <input
          type="file"
          id={"file"}
          onChange={(event: ChangeEvent<HTMLInputElement>) => {
            console.log("cool", event.target.files![0]);
            exec({ file: event.target.files![0] });
            // commit({
            //   variables: { file: event.target.files![0] },
            //   uploadables: { file: event.target.files![0] },
            //   onError(err) {
            //     console.log("err", err);
            //   },
            // });
          }}
        />
      </form>
    </div>
  );
}

export default Upload;
